use heck::ToShoutySnakeCase;
use quick_xml::{Reader, events::Event};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
struct Emoji {
    emoji: String,
    name: String,
    group: String,
    subgroup: String,
    codepoints: Vec<String>,
    qualification: String,
    emoji_version: String,
    const_name: String,
    is_flag: bool,
    has_skin_tone: bool,
    has_variation_selector: bool,
    normalized_name: String,
}

pub fn generate_emojis(
    version: &str,
    input: Option<PathBuf>,
    cldr: Option<PathBuf>,
    constants_path: &Path,
    details_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = input.unwrap_or_else(|| emoji_input_path(version));

    let input = fs::read_to_string(&input_path)?;
    let unicode_version =
        parse_header_value(&input, "# Version: ").unwrap_or_else(|| version.to_string());
    let mut emojis = parse_emoji_test(&input);

    let cldr_path = cldr.unwrap_or_else(|| cldr_input_path(version));
    let cldr = fs::read_to_string(&cldr_path)?;
    let cldr_version = parse_cldr_version(&cldr).unwrap_or_else(|| "unknown".to_string());
    let keywords = load_keywords(&cldr)?;

    assign_const_names(&mut emojis);

    write_constants(constants_path, &emojis, &unicode_version, &cldr_version)?;
    write_details(
        details_path,
        &emojis,
        &keywords,
        &unicode_version,
        &cldr_version,
    )?;

    println!(
        "generated {} emojis from {}",
        emojis.len(),
        input_path.display()
    );

    println!("wrote {}", constants_path.display());
    println!("wrote {}", details_path.display());

    Ok(())
}

fn emoji_input_path(version: &str) -> PathBuf {
    let path = if version.eq_ignore_ascii_case("latest") {
        PathBuf::from("data/unicode-emoji.txt")
    } else {
        PathBuf::from(format!("data/unicode-emoji-{version}.txt"))
    };

    if path.exists() {
        path
    } else if version.eq_ignore_ascii_case("latest") {
        PathBuf::from("workspace/data/unicode-emoji.txt")
    } else {
        PathBuf::from(format!("workspace/data/unicode-emoji-{version}.txt"))
    }
}

fn cldr_input_path(version: &str) -> PathBuf {
    let path = if version.eq_ignore_ascii_case("latest") {
        PathBuf::from("data/unicode-cldr-en.xml")
    } else {
        PathBuf::from(format!("data/unicode-cldr-en-{version}.xml"))
    };

    if path.exists() {
        path
    } else if version.eq_ignore_ascii_case("latest") {
        PathBuf::from("workspace/data/unicode-cldr-en.xml")
    } else {
        PathBuf::from(format!("workspace/data/unicode-cldr-en-{version}.xml"))
    }
}

fn parse_emoji_test(input: &str) -> Vec<Emoji> {
    let mut group = String::new();
    let mut subgroup = String::new();
    let mut emojis = Vec::new();

    for raw_line in input.lines() {
        let line = raw_line.trim();

        if let Some(value) = line.strip_prefix("# group: ") {
            group = value.to_string();
            continue;
        }

        if let Some(value) = line.strip_prefix("# subgroup: ") {
            subgroup = value.to_string();
            continue;
        }

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((left, right)) = line.split_once('#') else {
            continue;
        };

        let Some((codepoints_raw, status_raw)) = left.split_once(';') else {
            continue;
        };

        let qualification = status_raw.trim();

        if qualification != "fully-qualified" {
            continue;
        }

        let codepoints: Vec<String> = codepoints_raw
            .split_whitespace()
            .map(|cp| cp.to_ascii_uppercase())
            .collect();

        let mut comment_parts = right.trim().splitn(3, ' ');
        let emoji = comment_parts.next().unwrap_or_default().to_string();

        // Example: E1.0, E13.1, E17.0
        let emoji_version = comment_parts
            .next()
            .unwrap_or_default()
            .trim_start_matches('E')
            .to_string();
        let mut name = comment_parts.next().unwrap_or_default().to_string();

        // Some entries (notably flags) are prefixed with "flag: ".
        // Remember that this is a flag (for constant naming), but strip
        // the prefix so the details `name` contains only the country/place.
        let mut is_flag = false;
        if name.to_lowercase().starts_with("flag: ") {
            is_flag = true;
            name = name[6..].trim().to_string();
        }

        if emoji.is_empty() || name.is_empty() {
            continue;
        }

        let has_skin_tone = has_skin_tone_modifier(codepoints_raw);
        let has_variation_selector = has_variation_selector(codepoints_raw);
        let normalized_name = normalize_search_text(&name);

        emojis.push(Emoji {
            emoji,
            name,
            group: group.clone(),
            subgroup: subgroup.clone(),
            codepoints,
            qualification: qualification.to_string(),
            const_name: String::new(),
            is_flag,
            emoji_version,
            has_skin_tone,
            has_variation_selector,
            normalized_name,
        });
    }

    emojis
}

fn assign_const_names(emojis: &mut [Emoji]) {
    let mut seen: HashMap<String, usize> = HashMap::new();

    for emoji in emojis {
        let base = if emoji.is_flag {
            format!("EMOJI_FLAG_{}", emoji.name.to_shouty_snake_case())
        } else {
            format!("EMOJI_{}", emoji.name.to_shouty_snake_case())
        };
        let count = seen.entry(base.clone()).or_insert(0);

        emoji.const_name = if *count == 0 {
            base
        } else {
            format!("{}_{}", base, emoji.codepoints.join("_"))
        };

        *count += 1;
    }
}

fn load_keywords(xml: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut map = HashMap::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) if e.name().as_ref() == b"annotation" => {
                let mut cp = None;
                let mut tts = false;

                for attr in e.attributes() {
                    let attr = attr?;
                    match attr.key.as_ref() {
                        b"cp" => cp = Some(String::from_utf8(attr.value.into_owned())?),
                        b"type" if attr.value.as_ref() == b"tts" => tts = true,
                        _ => {}
                    }
                }

                let text = reader.read_text(e.name())?;

                if let Some(cp) = cp
                    && !tts
                {
                    let words = text
                        .split('|')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>();

                    map.entry(cp).or_insert(words);
                }
            }
            Event::Eof => break,
            _ => {}
        }

        buf.clear();
    }

    Ok(map)
}

fn write_constants(
    path: &Path,
    emojis: &[Emoji],
    unicode_version: &str,
    cldr_version: &str,
) -> std::io::Result<()> {
    let mut output = String::new();

    output.push_str("// @generated by veltrix-codegen. Do not edit.\n");
    output.push_str("#![allow(missing_docs)]\n\n");
    output.push_str(&format!(
        "// Source data: Unicode Emoji {unicode_version}, CLDR {cldr_version}.\n\n"
    ));

    for emoji in emojis {
        output.push_str(&format!(
            "pub const {}: &str = \"{}\";\n",
            emoji.const_name, emoji.emoji
        ));
    }

    write_file(path, output)
}

fn write_details(
    path: &Path,
    emojis: &[Emoji],
    keywords: &HashMap<String, Vec<String>>,
    unicode_version: &str,
    cldr_version: &str,
) -> std::io::Result<()> {
    let mut output = String::new();

    output.push_str("// @generated by veltrix-codegen. Do not edit.\n");
    output.push_str("#![allow(missing_docs)]\n\n");
    output.push_str("use super::constants::*;\n\n");
    output.push_str(&format!(
        "pub const UNICODE_EMOJI_VERSION: &str = {:?};\n",
        unicode_version
    ));
    output.push_str(&format!(
        "pub const CLDR_VERSION: &str = {:?};\n\n",
        cldr_version
    ));

    output.push_str(
        r#"/// Metadata for one generated Unicode emoji entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Emoji {
    pub emoji: &'static str,
    pub name: &'static str,
    pub group: &'static str,
    pub subgroup: &'static str,
    pub codepoints: &'static [&'static str],
    pub keywords: &'static [&'static str],
    pub search_terms: &'static [&'static str],
    pub qualification: &'static str,
    pub emoji_version: &'static str,
    pub unicode_version: &'static str,
    pub has_skin_tone: bool,
    pub has_variation_selector: bool,
    pub is_flag: bool,
    pub normalized_name: &'static str,
}

"#,
    );

    output.push_str("#[rustfmt::skip]\n");
    output.push_str("pub const ALL: &[Emoji] = &[\n");

    for emoji in emojis {
        output.push_str("    Emoji {\n");
        output.push_str(&format!("        emoji: {},\n", emoji.const_name));
        output.push_str(&format!("        name: {:?},\n", emoji.name));
        output.push_str(&format!("        group: {:?},\n", emoji.group));
        output.push_str(&format!("        subgroup: {:?},\n", emoji.subgroup));
        output.push_str(&format!(
            "        codepoints: &{},\n",
            rust_string_slice(&emoji.codepoints)
        ));
        let key = emoji.emoji.as_str();
        let words = keywords.get(key).map(Vec::as_slice).unwrap_or(&[]);
        let search_terms = search_terms(&emoji.normalized_name, words);

        output.push_str(&format!(
            "        keywords: &{},\n",
            rust_string_slice(words)
        ));
        output.push_str(&format!(
            "        search_terms: &{},\n",
            rust_string_slice(&search_terms)
        ));
        output.push_str(&format!(
            "        qualification: {:?},\n",
            emoji.qualification
        ));
        output.push_str(&format!(
            "        emoji_version: {:?},\n",
            emoji.emoji_version
        ));
        output.push_str("        unicode_version: UNICODE_EMOJI_VERSION,\n");
        output.push_str(&format!(
            "        has_skin_tone: {},\n",
            emoji.has_skin_tone
        ));
        output.push_str(&format!(
            "        has_variation_selector: {},\n",
            emoji.has_variation_selector
        ));
        output.push_str(&format!("        is_flag: {},\n", emoji.is_flag));
        output.push_str(&format!(
            "        normalized_name: {:?},\n",
            emoji.normalized_name
        ));
        output.push_str("    },\n");
    }

    output.push_str("];\n\n");

    output.push_str(
        r#"/// Find an emoji by its canonical Unicode name, case-insensitively.
pub fn find_by_name(name: &str) -> Option<&'static Emoji> {
    ALL.iter().find(|item| item.name.eq_ignore_ascii_case(name))
}

/// Find an emoji metadata entry by its rendered emoji string.
pub fn find_by_emoji(emoji: &str) -> Option<&'static Emoji> {
    ALL.iter().find(|item| item.emoji == emoji)
}

/// Find the first emoji matching a normalized name or keyword term.
pub fn find_by_search_term(term: &str) -> Option<&'static Emoji> {
    let normalized = normalize_search_term(term);

    ALL.iter().find(|item| {
        item.normalized_name == normalized
            || item.search_terms.iter().any(|candidate| *candidate == normalized)
    })
}

/// Iterate over all emoji entries in a Unicode group.
pub fn by_group(group: &str) -> impl Iterator<Item = &'static Emoji> {
    ALL.iter().filter(move |item| item.group == group)
}

/// Iterate over all emoji entries in a Unicode subgroup.
pub fn by_subgroup(subgroup: &str) -> impl Iterator<Item = &'static Emoji> {
    ALL.iter().filter(move |item| item.subgroup == subgroup)
}

fn normalize_search_term(term: &str) -> String {
    term.chars()
        .flat_map(char::to_lowercase)
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_metadata_records_source_versions() {
        assert_eq!(UNICODE_EMOJI_VERSION, "17.0");
        assert_eq!(CLDR_VERSION, "48.2");
        assert!(ALL.iter().all(|item| item.unicode_version == UNICODE_EMOJI_VERSION));
    }

    #[test]
    fn generated_metadata_exposes_search_and_variation_fields() {
        let smiling = find_by_name("smiling face").expect("smiling face exists");

        assert!(smiling.has_variation_selector);
        assert_eq!(smiling.normalized_name, "smiling face");
        assert!(find_by_search_term("smile").is_some());
    }
}
"#,
    );

    write_file(path, output)
}

fn write_file(path: &Path, contents: String) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, contents)
}

fn parse_header_value(input: &str, prefix: &str) -> Option<String> {
    input
        .lines()
        .find_map(|line| line.trim().strip_prefix(prefix).map(str::to_string))
}

fn parse_cldr_version(xml: &str) -> Option<String> {
    let marker = "<version number=\"";
    let start = xml.find(marker)? + marker.len();
    let end = xml[start..].find('"')?;
    Some(xml[start..start + end].to_string())
}

fn has_skin_tone_modifier(codepoints_raw: &str) -> bool {
    codepoints_raw.split_whitespace().any(|cp| {
        matches!(
            cp.to_ascii_uppercase().as_str(),
            "1F3FB" | "1F3FC" | "1F3FD" | "1F3FE" | "1F3FF"
        )
    })
}

fn has_variation_selector(codepoints_raw: &str) -> bool {
    codepoints_raw
        .split_whitespace()
        .any(|cp| matches!(cp.to_ascii_uppercase().as_str(), "FE0E" | "FE0F"))
}

fn normalize_search_text(value: &str) -> String {
    value
        .chars()
        .flat_map(char::to_lowercase)
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn search_terms(normalized_name: &str, keywords: &[String]) -> Vec<String> {
    let mut terms = Vec::new();

    for term in normalized_name.split_whitespace() {
        push_unique(&mut terms, term.to_string());
    }

    for keyword in keywords {
        let normalized = normalize_search_text(keyword);
        if !normalized.is_empty() {
            push_unique(&mut terms, normalized);
        }
    }

    terms
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.iter().any(|existing| existing == &value) {
        values.push(value);
    }
}

fn rust_string_slice(values: &[String]) -> String {
    let values = values
        .iter()
        .map(|value| format!("{value:?}"))
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{values}]")
}

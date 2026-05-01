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
    emoji_version: String,
    const_name: String,
    is_flag: bool,
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
    let mut emojis = parse_emoji_test(&input);

    let cldr_path = cldr.unwrap_or_else(|| cldr_input_path(version));
    let keywords = load_keywords(&cldr_path)?;

    assign_const_names(&mut emojis);

    write_constants(constants_path, &emojis)?;
    write_details(details_path, &emojis, &keywords)?;

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
    if version.eq_ignore_ascii_case("latest") {
        PathBuf::from("data/unicode-emoji.txt")
    } else {
        PathBuf::from(format!("data/unicode-emoji-{version}.txt"))
    }
}

fn cldr_input_path(version: &str) -> PathBuf {
    if version.eq_ignore_ascii_case("latest") {
        PathBuf::from("data/unicode-cldr-en.xml")
    } else {
        PathBuf::from(format!("data/unicode-cldr-en-{version}.xml"))
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

        if status_raw.trim() != "fully-qualified" {
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

        emojis.push(Emoji {
            emoji,
            name,
            group: group.clone(),
            subgroup: subgroup.clone(),
            codepoints,
            const_name: String::new(),
            is_flag,
            emoji_version,
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

fn load_keywords(path: &Path) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let xml = fs::read_to_string(path)?;
    let mut reader = Reader::from_str(&xml);
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

fn write_constants(path: &Path, emojis: &[Emoji]) -> std::io::Result<()> {
    let mut output = String::new();

    output.push_str("// @generated by veltrix-codegen. Do not edit.\n\n");

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
) -> std::io::Result<()> {
    let mut output = String::new();

    output.push_str("// @generated by veltrix-codegen. Do not edit.\n\n");
    output.push_str("use super::constants::*;\n\n");

    output.push_str(
        r#"#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Emoji {
    pub emoji: &'static str,
    pub name: &'static str,
    pub group: &'static str,
    pub subgroup: &'static str,
    pub codepoints: &'static [&'static str],
    pub keywords: &'static [&'static str],
    pub emoji_version: &'static str,
}

"#,
    );

    output.push_str("pub const ALL: &[Emoji] = &[\n");

    for emoji in emojis {
        output.push_str("    Emoji {\n");
        output.push_str(&format!("        emoji: {},\n", emoji.const_name));
        output.push_str(&format!("        name: {:?},\n", emoji.name));
        output.push_str(&format!("        group: {:?},\n", emoji.group));
        output.push_str(&format!("        subgroup: {:?},\n", emoji.subgroup));
        output.push_str("        codepoints: &[\n");

        for cp in &emoji.codepoints {
            output.push_str(&format!("            {:?},\n", cp));
        }

        output.push_str("        ],\n");
        let key = emoji.emoji.as_str();
        let words = keywords.get(key).map(Vec::as_slice).unwrap_or(&[]);

        output.push_str("        keywords: &[\n");
        for word in words {
            output.push_str(&format!("            {:?},\n", word));
        }
        output.push_str("        ],\n");
        output.push_str(&format!(
            "        emoji_version: {:?},\n",
            emoji.emoji_version
        ));
        output.push_str("    },\n");
    }

    output.push_str("];\n\n");

    output.push_str(
        r#"pub fn find_by_name(name: &str) -> Option<&'static Emoji> {
    ALL.iter().find(|item| item.name.eq_ignore_ascii_case(name))
}

pub fn find_by_emoji(emoji: &str) -> Option<&'static Emoji> {
    ALL.iter().find(|item| item.emoji == emoji)
}

pub fn by_group(group: &str) -> impl Iterator<Item = &'static Emoji> {
    ALL.iter().filter(move |item| item.group == group)
}

pub fn by_subgroup(subgroup: &str) -> impl Iterator<Item = &'static Emoji> {
    ALL.iter().filter(move |item| item.subgroup == subgroup)
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

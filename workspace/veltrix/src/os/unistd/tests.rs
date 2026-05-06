
use super::*;
use std::fs::File;

#[test]
fn chown_noop_for_current_user() {
    let mut path = std::env::temp_dir();
    path.push(format!("veltrix_chown_test_{}", std::process::id()));

    let _ = File::create(&path).expect("create temp file");

    let res = chown(
        &path,
        Some(super::getuid()),
        Some(super::getgid()),
    );

    let _ = std::fs::remove_file(&path);

    assert!(res.is_ok());
}

#[test]
fn fchown_noop_for_current_user() {
    let mut path = std::env::temp_dir();
    path.push(format!("veltrix_fchown_test_{}", std::process::id()));

    let _ = File::create(&path).expect("create temp file");
    let f = File::open(&path).expect("open file");

    let res = fchown(
        &f,
        Some(super::getuid()),
        Some(super::getgid()),
    );

    let _ = std::fs::remove_file(&path);

    assert!(res.is_ok());
}

#[test]
fn lchown_noop_for_current_user() {
    use std::os::unix::fs::symlink;

    let mut target = std::env::temp_dir();
    target.push(format!("veltrix_lchown_target_{}", std::process::id()));
    let _ = File::create(&target).expect("create target");

    let mut link = std::env::temp_dir();
    link.push(format!("veltrix_lchown_link_{}", std::process::id()));

    let _ = std::fs::remove_file(&link);

    symlink(&target, &link).expect("create symlink");

    let res = lchown(
        &link,
        Some(super::getuid()),
        Some(super::getgid()),
    );

    let _ = std::fs::remove_file(&link);
    let _ = std::fs::remove_file(&target);

    assert!(res.is_ok());
}

#[test]
fn chown_by_names_noop_for_current_user() {
    let mut path = std::env::temp_dir();
    path.push(format!("veltrix_chown_names_test_{}", std::process::id()));

    let _ = File::create(&path).expect("create temp file");

    let username = super::username_by_uid(super::getuid()).expect("username exists");
    let groupname = super::groupname_by_gid(super::getgid()).expect("group exists");

    let res = chown_by_names(&path, Some(&username), Some(&groupname));

    let _ = std::fs::remove_file(&path);

    assert!(res.is_ok());
}

#[test]
fn fchown_by_names_noop_for_current_user() {
    let mut path = std::env::temp_dir();
    path.push(format!("veltrix_fchown_names_test_{}", std::process::id()));

    let _ = File::create(&path).expect("create temp file");
    let f = File::open(&path).expect("open file");

    let username = super::username_by_uid(super::getuid()).expect("username exists");
    let groupname = super::groupname_by_gid(super::getgid()).expect("group exists");

    let res = fchown_by_names(&f, Some(&username), Some(&groupname));

    let _ = std::fs::remove_file(&path);

    assert!(res.is_ok());
}

#[test]
fn lchown_by_names_noop_for_current_user() {
    use std::os::unix::fs::symlink;

    let mut target = std::env::temp_dir();
    target.push(format!(
        "veltrix_lchown_names_target_{}",
        std::process::id()
    ));
    let _ = File::create(&target).expect("create target");

    let mut link = std::env::temp_dir();
    link.push(format!("veltrix_lchown_names_link_{}", std::process::id()));

    let _ = std::fs::remove_file(&link);

    symlink(&target, &link).expect("create symlink");

    let username = super::username_by_uid(super::getuid()).expect("username exists");
    let groupname = super::groupname_by_gid(super::getgid()).expect("group exists");

    let res = lchown_by_names(&link, Some(&username), Some(&groupname));

    let _ = std::fs::remove_file(&link);
    let _ = std::fs::remove_file(&target);

    assert!(res.is_ok());
}

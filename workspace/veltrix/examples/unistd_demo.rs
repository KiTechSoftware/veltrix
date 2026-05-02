#[cfg(feature = "unistd")]
use veltrix::os::unistd::{
    self, getcwd, getegid, geteuid, getgid, gethostname, getpid, getppid, getuid, groups_for_uid,
};

#[cfg(feature = "unistd")]
fn main() {
    let uid = getuid();
    let euid = geteuid();
    let gid = getgid();
    let egid = getegid();
    let pid = getpid();
    let ppid = getppid();

    println!("uid: {}  euid: {}", uid, euid);
    println!("gid: {}  egid: {}", gid, egid);
    println!("pid: {}  ppid: {}", pid, ppid);

    if let Some(name) = unistd::username_by_uid(uid) {
        println!("username: {}", name);
    }

    if let Ok(h) = gethostname() {
        println!("hostname: {}", h);
    }

    if let Ok(cwd) = getcwd() {
        println!("cwd: {}", cwd.display());
    }

    let groups = groups_for_uid(uid);
    println!("groups ({}): {:?}", groups.len(), groups);

    println!("is root: {}", unistd::is_root());
    println!("is effective root: {}", unistd::is_effective_root());
    println!("in admin group: {}", unistd::user_in_admin_group(uid));
}

#[cfg(not(feature = "unistd"))]
fn main() {
    eprintln!("Enable the `unistd` feature to run this example:");
    eprintln!(
        "  cargo run --manifest-path workspace/Cargo.toml -p veltrix --example unistd_demo --features unistd"
    );
}

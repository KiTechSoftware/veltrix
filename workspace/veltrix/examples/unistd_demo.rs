#[cfg(feature = "unistd")]
use veltrix::unistd::{
    self, getcwd, geteuid, gethostname, getpid, getppid, getuid, groups_for_uid,
};

#[cfg(feature = "unistd")]
fn main() {
    let uid = getuid();
    let euid = geteuid();
    let pid = getpid();
    let ppid = getppid();

    println!("uid: {}  euid: {}", uid, euid);
    println!("pid: {}  ppid: {}", pid, ppid);

    if let Some(name) = unistd::username_by_uid(uid) {
        println!("username: {}", name);
    }

    if let Some(h) = gethostname().ok() {
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
    eprintln!("  cargo run --example unistd_demo --features unistd");
}

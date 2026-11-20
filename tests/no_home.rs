
#[cfg(target_os = "linux")]
#[test]
fn runs_without_home_or_passwd_entry() {
    use std::process::Command;

    // Changing UID requires root or CAP_SETUID/CAP_SETGID.
    let uid = Command::new("id")
        .arg("-u")
        .output()
        .expect("failed to determine UID");

    if String::from_utf8_lossy(&uid.stdout).trim() != "0" {
        eprintln!("Skipping: requires root");
        return;
    }

    // Use a UID without a passwd/NSS entry.
    const TEST_UID: &str = "987654321";

    let lookup = Command::new("getent")
        .args(["passwd", TEST_UID])
        .status()
        .expect("getent is required for this test");

    assert!(
        !lookup.success(),
        "Test UID unexpectedly has a passwd entry"
    );

    let output = Command::new("setpriv")
        .args([
            "--reuid=987654321",
            "--regid=987654321",
            "--clear-groups",
            "env",
            "-i",
            env!("CARGO_BIN_EXE_c2patool"),
            "--version",
        ])
        .output()
        .expect("failed to execute c2patool");

    assert!(
        output.status.success(),
        "c2patool failed without HOME or passwd entry\n\
         exit: {:?}\nstdout: {}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    assert!(
        String::from_utf8_lossy(&output.stdout).contains("c2patool"),
        "expected c2patool version output"
    );
}


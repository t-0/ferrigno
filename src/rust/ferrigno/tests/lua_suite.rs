use std::process::Command;

fn run_ferrigno(args: &[&str]) -> std::process::Output {
    let bin = std::env::var("CARGO_BIN_EXE_ferrigno").unwrap_or_else(|_| {
        // Integration test binaries live in target/<profile>/deps/;
        // the ferrigno binary lives one level up in target/<profile>/.
        let test_exe = std::env::current_exe().expect("cannot locate test binary");
        let deps_dir = test_exe.parent().expect("no parent dir");
        let profile_dir = deps_dir.parent().expect("no profile dir");
        profile_dir.join("ferrigno").to_string_lossy().into_owned()
    });
    Command::new(bin)
        .args(args)
        .output()
        .expect("failed to run ferrigno")
}

#[test]
fn lua_basic() {
    let output = run_ferrigno(&["-e", "print('hello')"]);
    let sstdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "basic test failed: exit {:?}\nstdout: {}\nstderr: {}",
        output.status.code(),
        sstdout,
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(sstdout.contains("hello"));
}

#[test]
fn lua_test_suite() {
    let output = run_ferrigno(&["--bare", "@tests/all.lua"]);
    let sstdout = String::from_utf8_lossy(&output.stdout);
    let sstderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        eprintln!("stdout:\n{}", sstdout);
        eprintln!("stderr:\n{}", sstderr);
        panic!("Lua test suite failed (exit {:?})", output.status.code());
    }
    assert!(
        sstdout.contains("final OK"),
        "test suite did not reach 'final OK':\n{}",
        sstdout
    );
}

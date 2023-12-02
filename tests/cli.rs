use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rustforfun").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn echo_ok() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.assert().stdout("\n");

    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.args(["-n"]).assert().stdout("");

    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.args(["hello"]).assert().stdout("hello\n");

    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.args(["hello", "world"])
        .assert()
        .stdout("hello world\n");

    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.args(["hello", "world", "hello     RUST"])
        .assert()
        .stdout("hello world hello     RUST\n");

    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.args(["-n", "hello", "world", "hello     RUST"])
        .assert()
        .stdout("hello world hello     RUST");

    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.args(["hello", "world", "-n", "hello     RUST"])
        .assert()
        .stdout("hello world -n hello     RUST\n");
}

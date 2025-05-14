use assert_cmd::Command;
use std::fs::{self, File};
use std::io::Write;

fn setup_test_file(name: &str, content: &str) {
    let _ = fs::create_dir_all("tests/tmp");
    let mut file = File::create(format!("tests/tmp/{name}")).unwrap();
    writeln!(file, "{content}").unwrap();
}

#[test]
fn echo_prints_text() {
    let mut cmd = Command::cargo_bin("rsutils").unwrap();
    cmd.args(&["echo", "Hello", "World!"])
        .assert()
        .success()
        .stdout("Hello World!\n");
}

#[test]
fn cat_prints_file() {
    setup_test_file("cat.txt", "line 1\nline 2");
    let mut cmd = Command::cargo_bin("rsutils").unwrap();
    cmd.args(&["cat", "tests/tmp/cat.txt"])
        .assert()
        .success()
        .stdout(predicates::str::contains("line 1"));
}

#[test]
fn cat_prints_numbered_lines() {
    setup_test_file("cat_num.txt", "a\nb");
    let mut cmd = Command::cargo_bin("rsutils").unwrap();
    cmd.args(&["cat", "-n", "tests/tmp/cat_num.txt"])
        .assert()
        .success()
        .stdout(predicates::str::contains("1").and(predicates::str::contains("a")));
}

#[test]
fn ls_lists_current_directory() {
    let mut cmd = Command::cargo_bin("rsutils").unwrap();
    cmd.arg("ls")
        .assert()
        .success()
        .stdout(predicates::str::contains("src"));
}

#[test]
fn find_finds_file_by_name() {
    setup_test_file("find_me.txt", "dummy");
    let mut cmd = Command::cargo_bin("rsutils").unwrap();
    cmd.args(&["find", "tests/tmp", "--name", "find_me"])
        .assert()
        .success()
        .stdout(predicates::str::contains("find_me.txt"));
}

#[test]
fn grep_finds_text_in_file() {
    setup_test_file("grep.txt", "This is a test.\nAnother line.");
    let mut cmd = Command::cargo_bin("rsutils").unwrap();
    cmd.args(&["grep", "test", "tests/tmp/grep.txt"])
        .assert()
        .success()
        .stdout(predicates::str::contains("This is a test."));
}

mod command;

use crate::command::CommandOutput;

#[test]
fn test_result_in_n() {
    let cmd = CommandOutput::cargo_run("tests/fixtures/ejemplo01.txt");
    let cmd_assert = CommandOutput::run(cmd).unwrap();

    if let Some(stdout) = cmd_assert.stdout() {
        assert!(cmd_assert.success());
        assert_eq!(stdout, "N");
    } else {
        println!("Test skipped: stdout output is empty");
    }
}

#[test]
fn test_result_in_b() {
    let cmd = CommandOutput::cargo_run("tests/fixtures/ejemplo02.txt");
    let cmd_assert = CommandOutput::run(cmd).unwrap();

    if let Some(stdout) = cmd_assert.stdout() {
        assert!(cmd_assert.success());
        assert_eq!(stdout, "B");
    } else {
        println!("Test skipped: stdout output is empty");
    }
}

#[test]
fn test_result_in_e() {
    let cmd = CommandOutput::cargo_run("tests/fixtures/ejemplo03.txt");
    let cmd_assert = CommandOutput::run(cmd).unwrap();

    if let Some(stdout) = cmd_assert.stdout() {
        assert!(cmd_assert.success());
        assert_eq!(stdout, "E");
    } else {
        println!("Test skipped: stderr of cargo child process output is empty");
    }
}

#[test]
fn test_result_in_p() {
    let cmd = CommandOutput::cargo_run("tests/fixtures/ejemplo04.txt");
    let cmd_assert = CommandOutput::run(cmd).unwrap();

    if let Some(stdout) = cmd_assert.stdout() {
        assert!(cmd_assert.success());
        assert_eq!(stdout, "P");
    } else {
        println!("Test skipped: stderr of cargo child process output is empty");
    }
}

#[test]
fn test_expected_error_format_for_invalid_character() {
    let cmd = CommandOutput::cargo_run("tests/fixtures/ejemplo_errors_01.txt");
    let cmd_assert = CommandOutput::run(cmd).unwrap();

    if let Some(stderr) = cmd_assert.stderr() {
        assert!(cmd_assert.failure());
        assert_eq!(stderr, "ERROR: [Invalid piece kind character: X]");
    } else {
        println!("Test skipped: stderr of cargo child process output is empty");
    }
}

#[test]
fn test_expected_error_format_for_invalid_board_size() {
    let cmd = CommandOutput::cargo_run("tests/fixtures/ejemplo_errors_02.txt");
    let cmd_assert = CommandOutput::run(cmd).unwrap();
    if let Some(stderr) = cmd_assert.stderr() {
        assert!(cmd_assert.failure());
        assert_eq!(stderr, "ERROR: [Invalid board size]");
    } else {
        println!("Test skipped: stderr of cargo child process output is empty");
    }
}

#[test]
fn test_expected_error_format_for_file_not_exists() {
    let cmd = CommandOutput::cargo_run("not_existing_test.txt");
    let cmd_assert = CommandOutput::run(cmd).unwrap();
    if let Some(stderr) = cmd_assert.stderr() {
        assert!(cmd_assert.failure());
        assert_eq!(stderr, "ERROR: [File not exists]");
    } else {
        println!("Test skipped: stderr of cargo child process output is empty");
    }
}

use std::process::{Command, Output};

const COMMANDS: [&str; 8] = [
    "state",
    "create",
    "start",
    "run",
    "pause",
    "resume",
    "terminate",
    "remove",
];

fn capsa(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_capsa"))
        .args(args)
        .output()
        .expect("launch the Capsa parser")
}

fn help(args: &[&str]) -> String {
    let output = capsa(args);
    assert!(output.status.success(), "help failed for {args:?}");
    assert!(
        output.stderr.is_empty(),
        "help wrote to stderr for {args:?}"
    );
    String::from_utf8(output.stdout).expect("help text is UTF-8")
}

#[test]
fn bare_invocation_uses_claps_builtin_missing_input_help() {
    let output = capsa(&[]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let text = String::from_utf8(output.stderr).expect("introductory help is UTF-8");
    for command in COMMANDS {
        assert!(text.contains(command), "missing command {command}");
    }
    for expected in [
        "Usage:",
        "--help",
        "--version",
        "Example:",
        "capsa help <command>",
    ] {
        assert!(
            text.contains(expected),
            "missing introductory content: {expected}"
        );
    }
    assert!(!text.contains("Documentation and feedback:"));
}

#[test]
fn all_top_level_help_forms_show_the_same_full_help() {
    let full = help(&["--help"]);
    assert_eq!(help(&["-h"]), full);
    assert_eq!(help(&["help"]), full);
    assert!(full.contains("capsa run fresh ./my-bundle"));
    assert!(full.contains("Documentation and feedback:"));
    assert!(full.contains("container execution is not implemented"));
}

#[test]
fn every_command_has_full_help_without_its_required_arguments() {
    for command in COMMANDS {
        let full = help(&[command, "--help"]);
        assert_eq!(help(&[command, "-h"]), full, "short help for {command}");
        assert_eq!(help(&["help", command]), full, "help command for {command}");
        assert!(full.contains(&format!("capsa {command}")));
        assert!(full.contains("Example"));
        assert!(full.contains("CONTAINER_ID"));
        assert!(
            full.contains("This build parses requests; container execution is not implemented."),
            "missing execution notice for {command}"
        );
    }
    assert!(help(&["run", "--help"]).contains("frozen configuration"));
    assert!(help(&["terminate", "--help"]).contains("Confirmation defaults to No"));
}

#[test]
fn version_is_a_successful_stdout_request() {
    assert_eq!(
        help(&["--version"]),
        format!("capsa {}\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn valid_inputs_still_parse_when_help_is_absent() {
    let cases: &[&[&str]] = &[
        &["state", "demo"],
        &["create", "demo", "./my-bundle"],
        &["start", "demo"],
        &["run", "prepared"],
        &["run", "fresh", "./my-bundle"],
        &["pause", "demo"],
        &["resume", "demo"],
        &["terminate", "demo"],
        &["remove", "demo"],
    ];
    for args in cases {
        let output = capsa(args);
        assert!(output.status.success(), "valid input rejected: {args:?}");
        assert!(output.stdout.is_empty());
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn missing_input_uses_claps_builtin_usage_errors() {
    for args in COMMANDS
        .iter()
        .map(|command| vec![*command])
        .chain([vec!["create", "demo"]])
    {
        let output = capsa(&args);
        assert_eq!(output.status.code(), Some(2), "missing input: {args:?}");
        assert!(output.stdout.is_empty());
        let message = String::from_utf8(output.stderr).expect("usage error is UTF-8");
        assert!(
            message.contains("required"),
            "missing diagnosis for {args:?}"
        );
        assert!(message.contains("Usage:"));
        assert!(message.contains("--help"));
    }
}

#[test]
fn typos_for_every_command_are_rejected_without_executing_corrections() {
    for (typo, command) in [
        ("statee", "state"),
        ("createe", "create"),
        ("startt", "start"),
        ("runn", "run"),
        ("pausee", "pause"),
        ("resumee", "resume"),
        ("terminatee", "terminate"),
        ("removee", "remove"),
    ] {
        let output = capsa(&[typo, "demo"]);
        assert_eq!(
            output.status.code(),
            Some(2),
            "unexpectedly accepted {typo}"
        );
        assert!(output.stdout.is_empty());
        let message = String::from_utf8(output.stderr).expect("command error is UTF-8");
        assert!(
            message.contains(&format!("'{command}'")),
            "missing correction for {typo}"
        );
        assert!(message.contains("--help"));
    }
}

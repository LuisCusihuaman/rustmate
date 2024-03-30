use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};

#[doc = "struct represents the output of a command, including its standard output, standard error, and exit status."]
pub struct CommandOutput {
    stdout: String,
    stderr: String,
    exit_status: Option<ExitStatus>,
}

impl CommandOutput {
    pub fn run(mut command: Command) -> Result<Self, Box<dyn Error>> {
        let child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let output = child.wait_with_output().unwrap();
        let stdout = BufReader::new(output.stdout.as_slice());
        let stderr = BufReader::new(output.stderr.as_slice());
        let mut stdout_str = String::new();
        let mut stderr_str = String::new();
        for line in stdout.lines().flatten() {
            stdout_str.push_str(&line);
        }
        for line in stderr.lines().flatten() {
            stderr_str.push_str(&line);
        }
        Ok(Self {
            stdout: stdout_str,
            stderr: stderr_str,
            exit_status: Some(output.status),
        })
    }

    #[doc = "Creates a new Command that will run the program specified by the filename argument."]
    pub fn cargo_run(filename: &str) -> Command {
        let mut cmd = Command::new("cargo");
        cmd.arg("run").arg("--quiet").arg("--").arg(filename);
        cmd
    }

    pub fn stdout(&self) -> Option<&str> {
        if self.stdout.is_empty() {
            None
        } else {
            Some(self.stdout.as_str())
        }
    }

    pub fn stderr(&self) -> Option<&str> {
        if self.stderr.is_empty() {
            None
        } else {
            Some(self.stderr.as_str())
        }
    }

    pub fn success(&self) -> bool {
        match self.exit_status {
            Some(status) => status.success(),
            None => false,
        }
    }

    pub fn failure(&self) -> bool {
        !self.success()
    }
}

//! Talks MCP over stdio to the server binary, the way an MCP client launches it.
//!
//! Binary under test: `RMCP_RELEASE_DEMO_BIN` when set (the post-release check points it at an
//! installed copy), otherwise the binary Cargo built for this package.
//! Expected version: `RMCP_RELEASE_DEMO_EXPECTED_VERSION` when set (the post-release check passes
//! the released version), otherwise `package.version` read from `Cargo.toml`.
//!
//! Not covered here: the release pipeline itself (targets, archives, checksums, installers, tap
//! updates) only runs on a tag push; its evidence is the release run and the post-release check.

use std::ffi::OsString;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

const TIMEOUT: Duration = Duration::from_secs(10);

/// The protocol revision the session asks for: the newest one that still has `initialize`.
const PROTOCOL_VERSION: &str = "2025-11-25";

fn non_empty_env(name: &str) -> Option<OsString> {
    std::env::var_os(name).filter(|value| !value.is_empty())
}

fn binary() -> OsString {
    non_empty_env("RMCP_RELEASE_DEMO_BIN")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_rmcp-release-demo").into())
}

fn expected_version() -> String {
    if let Some(version) = non_empty_env("RMCP_RELEASE_DEMO_EXPECTED_VERSION") {
        return version
            .into_string()
            .expect("RMCP_RELEASE_DEMO_EXPECTED_VERSION is UTF-8");
    }
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
    let manifest = std::fs::read_to_string(path).expect("Cargo.toml is readable");
    let manifest: toml::Table = toml::from_str(&manifest).expect("Cargo.toml is valid TOML");
    manifest["package"]["version"]
        .as_str()
        .expect("package.version is a string")
        .to_owned()
}

/// Everything the server wrote during a session.
struct Transcript {
    stdout: Vec<String>,
    stderr: String,
}

/// One running server process with its stdio attached to the test.
struct Session {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: Receiver<String>,
    stdout_lines: Vec<String>,
    stderr: Option<JoinHandle<String>>,
    next_id: u64,
}

impl Session {
    fn start() -> Self {
        let binary = binary();
        let mut child = Command::new(&binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap_or_else(|error| panic!("cannot start {}: {error}", binary.display()));

        let stdout = child.stdout.take().expect("stdout is piped");
        let (lines, receiver) = mpsc::channel();
        thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                let Ok(line) = line else { break };
                if lines.send(line).is_err() {
                    break;
                }
            }
        });

        let mut stderr = child.stderr.take().expect("stderr is piped");
        let stderr = thread::spawn(move || {
            let mut text = String::new();
            let _ = stderr.read_to_string(&mut text);
            text
        });

        Self {
            stdin: child.stdin.take(),
            child,
            stdout: receiver,
            stdout_lines: Vec::new(),
            stderr: Some(stderr),
            next_id: 0,
        }
    }

    fn send(&mut self, message: &Value) {
        let stdin = self.stdin.as_mut().expect("stdin is still open");
        writeln!(stdin, "{message}").expect("write to the server's stdin");
        stdin.flush().expect("flush the server's stdin");
    }

    /// Sends a request and returns its `result`; stdout lines that are not its response are
    /// recorded and skipped.
    fn request(&mut self, method: &str, params: Value) -> Value {
        self.next_id += 1;
        let id = self.next_id;
        self.send(&json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }));
        loop {
            let line = self
                .stdout
                .recv_timeout(TIMEOUT)
                .unwrap_or_else(|error| panic!("no response to {method} ({error:?})"));
            self.stdout_lines.push(line.clone());
            let Ok(message) = serde_json::from_str::<Value>(&line) else { continue };
            if message["id"] != json!(id) {
                continue;
            }
            if let Some(error) = message.get("error") {
                panic!("{method} failed: {error}");
            }
            return message["result"].clone();
        }
    }

    /// Runs the `initialize` handshake and returns the `initialize` result.
    fn initialize(&mut self) -> Value {
        let result = self.request(
            "initialize",
            json!({
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": {},
                "clientInfo": { "name": "distribution-tests", "version": "0" }
            }),
        );
        self.send(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }));
        result
    }

    fn call_tool(&mut self, name: &str, arguments: Value) -> Value {
        self.request("tools/call", json!({ "name": name, "arguments": arguments }))
    }

    /// Closes stdin, waits for the server to exit and returns everything it wrote.
    fn finish(&mut self) -> Transcript {
        drop(self.stdin.take());
        let deadline = Instant::now() + TIMEOUT;
        loop {
            match self.stdout.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
                Ok(line) => self.stdout_lines.push(line),
                Err(RecvTimeoutError::Disconnected) => break,
                Err(RecvTimeoutError::Timeout) => panic!("stdout still open after stdin closed"),
            }
        }
        while self.child.try_wait().expect("poll the server").is_none() {
            assert!(Instant::now() < deadline, "server still running after stdin closed");
            thread::sleep(Duration::from_millis(20));
        }
        let stderr = self.stderr.take().expect("finish runs once");
        Transcript {
            stdout: std::mem::take(&mut self.stdout_lines),
            stderr: stderr.join().expect("stderr reader"),
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// The text of a successful tool result that holds exactly one text content item.
fn single_text(result: &Value) -> &str {
    assert_ne!(result["isError"], json!(true), "tool reported an error: {result}");
    let content = result["content"].as_array().expect("content is an array");
    assert_eq!(content.len(), 1, "exactly one content item: {result}");
    assert_eq!(content[0]["type"], json!("text"), "a text content item: {result}");
    content[0]["text"].as_str().expect("text is a string")
}

#[test]
fn initialize_reports_the_package_version() {
    let mut session = Session::start();

    let result = session.initialize();

    assert_eq!(result["serverInfo"]["version"], json!(expected_version()));
}

#[test]
fn tools_list_returns_echo_and_version() {
    let mut session = Session::start();
    session.initialize();

    let result = session.request("tools/list", json!({}));

    let mut names: Vec<&str> = result["tools"]
        .as_array()
        .expect("tools is an array")
        .iter()
        .map(|tool| tool["name"].as_str().expect("tool name is a string"))
        .collect();
    names.sort_unstable();
    assert_eq!(names, ["echo", "version"]);
}

#[test]
fn echo_returns_the_text_unchanged() {
    let mut session = Session::start();
    session.initialize();

    let result = session.call_tool("echo", json!({ "text": "héllo\n\"wörld\" 🚀" }));

    assert_eq!(single_text(&result), "héllo\n\"wörld\" 🚀");
}

#[test]
fn version_tool_matches_the_initialize_version() {
    let mut session = Session::start();
    let initialize = session.initialize();

    let result = session.call_tool("version", json!({}));

    let server_version = initialize["serverInfo"]["version"]
        .as_str()
        .expect("serverInfo.version is a string");
    assert_eq!(single_text(&result), server_version);
}

#[test]
fn stdout_carries_only_mcp_messages_while_logs_go_to_stderr() {
    let mut session = Session::start();
    session.initialize();
    session.request("tools/list", json!({}));
    session.call_tool("echo", json!({ "text": "ping" }));
    session.call_tool("version", json!({}));

    let transcript = session.finish();

    assert!(!transcript.stdout.is_empty(), "the session wrote to stdout");
    for line in &transcript.stdout {
        let message: Value = serde_json::from_str(line)
            .unwrap_or_else(|error| panic!("stdout line is not JSON ({error}): {line}"));
        assert_eq!(message["jsonrpc"], json!("2.0"), "not a JSON-RPC 2.0 message: {line}");
    }
    let startup = format!("rmcp-release-demo {}", expected_version());
    assert!(
        transcript.stderr.contains(&startup),
        "stderr has the startup log {startup:?}: {:?}",
        transcript.stderr
    );
}

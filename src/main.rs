use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::{Command, Stdio};

#[derive(Debug, Serialize, Deserialize)]
struct Window {
    id: i64,
    #[serde(rename(deserialize = "is-visible"))]
    visible: bool,
    frame: Frame,
}

#[derive(Debug, Serialize, Deserialize)]
struct Frame {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

fn main() {
    let direction = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("next"));

    let windows_output = Command::new("yabai")
        .arg("-m")
        .arg("query")
        .arg("--windows")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute yabai command");

    let windows: Vec<Window> =
        serde_json::from_slice(&windows_output.stdout).expect("Failed to parse yabai JSON output");

    let mut visible_windows = Vec::new();

    for window in &windows {
        if window.visible {
            visible_windows.push(window);
        }
    }

    // Sort visible windows by ascending x and y coordinates
    visible_windows.sort_unstable_by(|a, b| {
        if a.frame.x == b.frame.x {
            a.frame.y.partial_cmp(&b.frame.y).unwrap()
        } else {
            a.frame.x.partial_cmp(&b.frame.x).unwrap()
        }
    });

    let current_window_id = Command::new("yabai")
        .arg("-m")
        .arg("query")
        .arg("--windows")
        .arg("--window")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute yabai command")
        .stdout;

    let current_window: Value =
        serde_json::from_slice(&current_window_id).expect("Failed to parse yabai JSON output");

    let current_id = current_window["id"].as_i64().unwrap();

    let current_index = visible_windows
        .iter()
        .position(|window| window.id == current_id)
        .unwrap();

    let next_index = match direction.as_str() {
        "prev" => (current_index + visible_windows.len() - 1) % visible_windows.len(),
        _ => (current_index + 1) % visible_windows.len(),
    };

    let next_window_id = visible_windows[next_index].id;

    Command::new("yabai")
        .arg("-m")
        .arg("window")
        .arg("--focus")
        .arg(format!("{}", next_window_id))
        .status()
        .expect("Failed to execute yabai command");
}

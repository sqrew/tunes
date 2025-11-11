//! Compilation and execution runner for live coding

use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

/// Find the tunes project root by looking for Cargo.toml
fn find_tunes_root() -> anyhow::Result<PathBuf> {
    // Start from current directory and walk up
    let mut current = std::env::current_dir()?;

    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            // Check if this is the tunes Cargo.toml
            let content = fs::read_to_string(&cargo_toml)?;
            if content.contains("name = \"tunes\"") {
                return Ok(current);
            }
        }

        // Go up one directory
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            return Err(anyhow::anyhow!(
                "Could not find tunes project root. Make sure you're running from within the tunes directory."
            ));
        }
    }
}

pub struct LiveRunner {
    source_file: PathBuf,
    temp_dir: PathBuf,
    current_process: Option<Child>,
}

impl LiveRunner {
    pub fn new(source_file: PathBuf) -> anyhow::Result<Self> {
        let temp_dir = std::env::temp_dir().join("tunes_live");
        fs::create_dir_all(&temp_dir)?;

        Ok(Self {
            source_file,
            temp_dir,
            current_process: None,
        })
    }

    /// Compile and run the user's live coding script
    pub fn compile_and_run(&mut self) -> anyhow::Result<()> {
        println!("🔨 Compiling {}...", self.source_file.display());

        // Stop current process if running
        self.stop();

        // Create a temporary Cargo project
        let project_dir = self.temp_dir.join("live_project");
        fs::create_dir_all(&project_dir)?;

        // Find the tunes project root by looking for Cargo.toml with [package] name = "tunes"
        let tunes_root = find_tunes_root()?;

        // Create Cargo.toml with live profile for fast iteration
        let cargo_toml = format!(
            r#"[package]
name = "tunes-live-session"
version = "0.1.0"
edition = "2021"

[dependencies]
tunes = {{ path = "{}" }}
anyhow = "1.0"

[profile.live]
inherits = "release"
opt-level = 2
lto = false
incremental = true
codegen-units = 256
"#,
            tunes_root.display()
        );
        fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;

        // Create src directory and copy user's file
        let src_dir = project_dir.join("src");
        fs::create_dir_all(&src_dir)?;

        // Read the source file and convert crate:: imports to tunes::
        // This allows editing with IDE support in src/templates/ while
        // still working when run as a standalone binary
        let mut source_content = fs::read_to_string(&self.source_file)?;
        source_content = source_content
            .replace("use crate::", "use tunes::")
            .replace("use crate::composition", "use tunes::composition")
            .replace("use crate::consts", "use tunes::consts")
            .replace("use crate::engine", "use tunes::engine")
            .replace("use crate::instruments", "use tunes::instruments")
            .replace("use crate::prelude", "use tunes::prelude");
        fs::write(src_dir.join("main.rs"), source_content)?;

        // Compile using live profile (fast iteration with good audio performance)
        let compile_output = Command::new("cargo")
            .arg("build")
            .arg("--profile")
            .arg("live")
            .current_dir(&project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !compile_output.status.success() {
            let stderr = String::from_utf8_lossy(&compile_output.stderr);
            println!("❌ Compilation failed:\n{}", stderr);
            return Err(anyhow::anyhow!("Compilation failed"));
        }

        println!("✅ Compiled successfully!");

        // Run the compiled binary (live profile outputs to target/live)
        let binary_path = project_dir
            .join("target/live")
            .join("tunes-live-session");

        println!("▶️  Starting playback...");

        let child = Command::new(binary_path)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?;

        self.current_process = Some(child);

        Ok(())
    }

    /// Stop the currently running process
    pub fn stop(&mut self) {
        if let Some(mut process) = self.current_process.take() {
            println!("⏹  Stopping current session...");

            // Give it a moment to finish current audio buffer
            std::thread::sleep(std::time::Duration::from_millis(100));

            let _ = process.kill();
            let _ = process.wait();

            // Small delay to let audio system settle
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }

    /// Check if the process is still running
    pub fn is_running(&mut self) -> bool {
        if let Some(ref mut process) = self.current_process {
            matches!(process.try_wait(), Ok(None))
        } else {
            false
        }
    }
}

impl Drop for LiveRunner {
    fn drop(&mut self) {
        self.stop();
    }
}

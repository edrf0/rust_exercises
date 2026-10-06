use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn create_sample_logs(dir_path: &Path) -> io::Result<Vec<PathBuf>> {
    fs::create_dir_all(dir_path)?;
    let app_path = dir_path.join("app.log");
    let error_path = dir_path.join("error.log");
    fs::write(&app_path, "...")?;
    fs::write(&error_path, "...")?;
    Ok(vec![app_path, error_path])
}

pub fn audit_logs(files: &[PathBuf]) -> io::Result<u64> {
    let total_bytes = files.iter()
        .filter_map(|file| {
            if !file.exists() {
                return None;
            }
            Some(file.metadata().ok()?.len())
        })
        .sum();
    Ok(total_bytes)
}

pub fn run_echo_check(message: &str) -> io::Result<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &format!("echo {}", message)])
            .output()?
    } else {
        Command::new("echo")
            .arg(message)
            .output()?
    };
    let processed_output = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(processed_output)
}

fn main() -> io::Result<()> {
    let temp_dir = std::env::temp_dir().join("rust_std_fs_tour");

    // Test 1: Create logs
    let files = create_sample_logs(&temp_dir)?;
    assert_eq!(files.len(), 2);
    assert!(files[0].exists());
    assert!(files[1].exists());

    // Test 2: Audit file size
    let total_size = audit_logs(&files)?;
    assert!(total_size > 0);

    // Test 3: Process execution
    let echo_result = run_echo_check("Rust process execution works!")?;
    assert_eq!(echo_result, "Rust process execution works!");

    // Clean up temporary files
    let _ = fs::remove_dir_all(temp_dir);

    println!("Success! std::fs and std::process exercises passed successfully.");
    Ok(())
}
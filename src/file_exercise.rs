use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// 1. Write text content to a file path.
pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

/// 2. Read the entire content of a file into a String.
pub fn read_file_content<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

/// 3. Append text to a file (creating it if missing).
pub fn append_to_file<P: AsRef<Path>>(path: P, extra_content: &str) -> io::Result<()> {
    let mut handle = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    handle.write_all(extra_content.as_bytes())?;
    handle.flush()?;
    Ok(())
}

/// 4. Get the size of the file in bytes.
pub fn get_file_size_bytes<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

fn main() -> io::Result<()> {
    let path = "test_output.txt";
    write_file(path, "Hello, Rust Filesystem!\n")?;
    if let Ok(content) = read_file_content(path) {
        println!("File Content:\n{}", content);
    }
    fs::remove_file(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_operations() {
        let path = "temp_test_file.txt";

        // Clean up beforehand just in case
        let _ = fs::remove_file(path);

        assert!(write_file(path, "Line 1\n").is_ok());

        let content = read_file_content(path).unwrap();
        assert_eq!(content, "Line 1\n");

        assert!(append_to_file(path, "Line 2\n").is_ok());

        let size = get_file_size_bytes(path).unwrap();
        assert!(size > 0);

        let final_content = read_file_content(path).unwrap();
        assert!(final_content.contains("Line 1"));
        assert!(final_content.contains("Line 2"));

        // Cleanup
        let _ = fs::remove_file(path);
    }
}
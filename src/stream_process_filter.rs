use std::io::{self, BufRead, Write};

/// 1. Count total lines and total words from any BufRead source.
pub fn count_lines_and_words<R: BufRead>(reader: R) -> io::Result<(usize, usize)> {
    reader
        .lines()
        .try_fold((0usize, 0usize), |(mut a, mut b), line| {
            if line.is_err() {
                return None;
            }
            a += 1;
            b += line.unwrap().split_whitespace().count();
            Some((a,b))
        })
        .ok_or(io::Error::new(io::ErrorKind::InvalidData, ""))
}

/// 2. Read lines from a BufRead source, filter out lines starting with `#`,
/// and write the clean lines to a Write destination.
pub fn filter_comments<R: BufRead, W: Write>(reader: R, mut writer: W) -> io::Result<()> {
    let clean_lines_vec: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with('#')
        })
        .collect();
    Ok(writeln!(writer, "{}", clean_lines_vec.join("\n"))?)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines_and_words() {
        let data = "line one\nline two three\n";
        let (lines, words) = count_lines_and_words(data.as_bytes()).unwrap();
        assert_eq!(lines, 2);
        assert_eq!(words, 5);
    }

    #[test]
    fn test_filter_comments() {
        let input = "# Header comment\nValid data 1\n  # Indented comment\nValid data 2\n";
        let mut output = Vec::new();

        filter_comments(input.as_bytes(), &mut output).unwrap();
        let result_str = String::from_utf8(output).unwrap();

        assert!(!result_str.contains("comment"));
        assert!(result_str.contains("Valid data 1"));
        assert!(result_str.contains("Valid data 2"));
    }
}
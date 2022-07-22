#[allow(dead_code)]
pub mod lib {
    use std::{fs, io::Error};

    use regex::Regex;

    pub fn read_file(path: &str) -> Result<String, Error> {
        fs::read_to_string(path)
    }

    pub fn read_line_count(path: &str) -> usize {
        read_file(path).unwrap().lines().count()
    }

    pub fn read_word_count(path: &str) -> usize {
        let re = Regex::new("").unwrap();
        let mut count = 0;
        for _ in read_file(path).unwrap().split(" ") {
            if re.is_match("[a-zA-Z0-9]+([-_][a-zA-Z0-9]+)*") {
                count += 1
            }
        }
        count
    }

    pub fn read_char_count(path: &str) -> usize {
        read_file(path).unwrap().len()
    }

    pub fn read_byte_count(path: &str) -> usize {
        read_file(path).unwrap().as_bytes().to_vec().len()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::lib::*;

    #[test]
    fn test_read() {
        let mut file_ref = std::fs::File::create("test1.txt").expect("test: #1(test_read), failed on file write");
        file_ref
            .write_all("test\n".as_bytes())
            .expect("test: #1(test_read), failed on file write");

        let contents = match read_file("test1.txt") {
            Ok(data) => data,
            Err(_) => {
                println!("error: file not found");
                std::process::exit(1);
            }
        };

        assert_eq!("test\n", contents);
        std::fs::remove_file("test1.txt").unwrap();
    }

    #[test]
    fn read_byte_test() {
        let mut file_ref = std::fs::File::create("test2.txt")
            .expect("test: #2(read_byte_test), failed on file write");
        file_ref
            .write_all("test\n".as_bytes())
            .expect("test: #2(read_byte_test), failed on file write");

        assert_eq!(5, read_byte_count("test2.txt"));
        std::fs::remove_file("test2.txt").unwrap();
    }

    #[test]
    fn read_line_test() {
        let mut file_ref = std::fs::File::create("test3.txt")
            .expect("test: #3(read_line_test), failed on file write");
        file_ref
            .write_all("test\n test test\n\n\n  test test".as_bytes())
            .expect("test: #3(read_line_test), failed on file write");

        assert_eq!(5, read_line_count("test3.txt"));
        std::fs::remove_file("test3.txt").unwrap();
    }

    #[test]
    fn read_word_test() {
        let mut file_ref = std::fs::File::create("test4.txt")
            .expect("test: #4(read_word_test), failed on file write");
        file_ref
            .write_all("test testies test".as_bytes())
            .expect("test: #4(read_word_test), failed on file write");

        assert_eq!(3, read_word_count("test4.txt"));
        std::fs::remove_file("test4.txt").unwrap();
    }

    #[test]
    fn read_char_test() {
        let mut file_ref = std::fs::File::create("test5.txt")
            .expect("test: #5(read_char_test), failed on file write");
        file_ref
            .write_all("test test testing".as_bytes())
            .expect("test: #5(read_char_test), failed on file write");

        assert_eq!(17, read_char_count("test5.txt"));
        std::fs::remove_file("test5.txt").unwrap();
    }
}

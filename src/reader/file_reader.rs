use std;

pub struct FileReader;

impl FileReader {
    pub fn read_to_string(&self, path: &str) -> std::io::Result<String> {
        std::fs::read_to_string(path)
    }
}
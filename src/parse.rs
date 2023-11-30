use std::{path::Path, fs::read_to_string};

pub fn open_input<T: AsRef<Path>>(file_path: T) -> String {
    read_to_string(file_path).expect("failed to open file")
}

use std::fs::read_to_string;


pub fn read_file(file_path: &str) -> String {
    let input = read_to_string(file_path)
        .expect("Failed to read input file");
    input
}

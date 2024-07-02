use std::io::Stdin;

pub fn strip(string: &str) -> &str {
    string
        .strip_suffix("\r\n")
        .or(string.strip_suffix("\n"))
        .unwrap_or(&string)
}

pub fn input(stdin: &Stdin) -> String {
    let mut buffer: String = String::new();
    stdin.read_line(&mut buffer).unwrap();
    strip(&buffer).to_string()
}

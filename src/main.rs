use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let word = &args[1];
    let path_to_file = &args[2];
    let path = Path::new(path_to_file);
    let file = File::open(path)?;
    let highlighted_word = format!("\x1b[31m{}\x1b[0m", word); // Red color using ANSI escape codes
    let mut buf = io::BufReader::new(file);
    let mut line = String::new(); // Re-use the same string when reading lines to avoid allocating a new one each time
    while buf.read_line(&mut line).unwrap() > 0 {
        if line.contains(word) {
            let mut result = String::with_capacity(line.len());
            let mut start = 0;

            // slightly faster than using .replace in my tests
            while let Some(pos) = &line[start..].find(word) {
                result.push_str(&line[start..start + pos]);
                result.push_str(&highlighted_word);
                start += pos + word.len();
            }
            result.push_str(&line[start..]);

            print!("{}", result);
        }
        line.clear();
    }
    Ok(())
}

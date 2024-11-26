use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines(filename: &String) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    let path = Path::new(filename);
    if path.is_dir() {
        let msg = format!("File {filename} is a directory, expected a file");
        return Err(Error::new(io::ErrorKind::Other, msg))
    }
    Ok(io::BufReader::new(file).lines())
}

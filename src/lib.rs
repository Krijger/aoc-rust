use std::fs::{metadata, File};
use std::io::{self, BufRead, Error};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>,
{
    match metadata(&filename) {
        Ok(meta) if meta.is_file() => (),
        Ok(_) => return Err(Error::new(io::ErrorKind::Unsupported, "Expected a file, but got a directory")),
        Err(e) => return Err(Error::new(io::ErrorKind::Other, e)),
    }

    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


// #[cfg(test)]
pub mod test_util {
    use std::io::Result;
        
    pub fn io_lines_from(input: &str) -> impl Iterator<Item = Result<String>> + '_ {
        input.lines()
            .map(|line| line.trim())
            .filter(|&line| !line.is_empty())
            .map(|line| Result::Ok(String::from(line)))
    }

}

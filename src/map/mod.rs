pub struct Mapp<T>(Vec<Vec<T>>);

impl <T> Mapp<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.0.get(y)?.get(x)
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        match self.0.first() {
            None => 0,
            Some(row) => row.len()
        }
    }
}

impl <T> Mapp<T> {
    pub fn find<P>(&self, predicate: P) -> Option<(usize, usize)>
    where 
        P: Fn(&T) -> bool,
    {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if predicate(self.get(x, y)?) {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

impl Mapp<char> {
    pub fn print(&self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                print!("{}", self.get(x, y).unwrap_or(&' '));
            }
            println!();
        }
    }
}

pub fn read_map(lines: Vec<String>) -> Mapp<char> {
    let mut values = Vec::new();
    let mut width = 0;
    for (y, line) in lines.iter().enumerate() {
        let chars: Vec<_> = line.chars().collect();
        if y == 0 {
            width = chars.len();
        } else if chars.len() != width {
            panic!("Lines are not the same number of chars");
        }
        values.push(chars);
    }
    Mapp(values)
}

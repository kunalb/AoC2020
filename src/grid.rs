struct AsciiGrid {
    data: Vec<Vec<char>>,
}

impl AsciiGrid {
    fn new(buffer: &str) -> AsciiGrid {
        AsciiGrid {
            data: buffer
                .lines()
                .map(|x| x.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }

    fn get<T>(row: T, col: T) -> Option<char>
    where
        T: PartialOrd<usize>
    {
        None
    }

    fn set<T>(row: T, col: T) -> Option<char> {
        None
    }

    fn rows(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.data[0].len()
    }
}

impl fmt::Debug for AsciiGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            f.write_str(&String::from_iter(row))?;
            f.write_str("\n")?;
        }
        f.write_str("\n")
    }
}


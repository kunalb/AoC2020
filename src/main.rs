use std::io::{self, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("So it begins!");

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
    }
}

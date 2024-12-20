use std::fmt::Display;
use std::io::{self, Write};

pub fn input(msg: impl Display) -> io::Result<String> {
    print!("{}", msg.to_string());
    io::stdout().flush()?;

    let mut text = String::new();
    match io::stdin().read_line(&mut text) {
        Ok(_) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Input cannot be empty",
                ))
            } else {
                Ok(trimmed.to_string())
            }
        }
        Err(e) => {
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        }
    }
}

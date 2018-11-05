use std::fmt;
use std::process;

pub trait OrExit<T> {
    fn or_exit(self) -> T;
}

impl<T, E> OrExit<T> for Result<T, E>
where
    E: fmt::Display,
{
    fn or_exit(self) -> T {
        self.unwrap_or_else(|e: E| -> T {
            eprintln!("Error: {}", e);
            process::exit(1);
        })
    }
}

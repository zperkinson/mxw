use colored::Colorize;
use std::process;

pub trait None<T> {
    fn none(self, message: &str) -> T;
}

impl<T> None<T> for Option<T> {
    fn none(self, message: &str) -> T {
        if let Some(v) = self {
            return v;
        }

        println!("{}: {}", "error".bold().red(), message);
        process::exit(1);
    }
}

use std::error::Error;
mod lib;
fn main() -> Result<(), Box<Error>> {
    lib::notify("Hello", "World! 🌍")
}
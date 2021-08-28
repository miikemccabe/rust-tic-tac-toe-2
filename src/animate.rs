use std::{thread};
use std::time::Duration;
use std::io::stdout;
use std::io::Write;

pub fn staggered_display(s: &str) -> Result<(),String> {
    for c in s.chars() {
        print!("{}", c);
        stdout().flush().expect("Flush failed");
        thread::sleep(Duration::from_millis(1));
    }
    println!();
    Ok(())
}

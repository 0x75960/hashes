use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    midy::find(&buffer).into_iter().for_each(|x| println!("{}", x));
    Ok(())
}

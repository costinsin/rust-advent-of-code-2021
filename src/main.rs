use std::{error::Error, fs::File, io::stdin};
mod challenges;

fn main() -> Result<(), Box<dyn Error>> {
    let mut line = String::new();

    println!("Enter challenge number:");
    stdin().read_line(&mut line)?;
    let challenge = line.trim().parse::<u32>()?;

    let path = format!("src/input/day{}.in", line.trim());

    let mut input_file;

    match File::open(path.clone()) {
        Ok(open_file) => {
            input_file = open_file;
            println!("Input file used: {}", path);
        }
        Err(err) => {
            println!("Failed to open `{}` with error: {}", path, err);
            return Ok(());
        }
    }

    match challenge {
        1 => challenges::day1::part2(&mut input_file)?,
        2 => challenges::day2::part2(&mut input_file)?,
        _ => (),
    }

    Ok(())
}

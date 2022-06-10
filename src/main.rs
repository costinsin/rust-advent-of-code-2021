use std::{
    error::Error,
    fs::File,
    io::{stdin, Read},
};
mod challenges;

fn main() -> Result<(), Box<dyn Error>> {
    let mut line = String::new();

    println!("Enter challenge number:");
    stdin().read_line(&mut line)?;
    let challenge = line.trim().parse::<u32>()?;

    let path = format!("src/input/day{}.in", line.trim());

    drop(line);

    let mut file_content = String::new();

    match File::open(path.clone()) {
        Ok(mut open_file) => {
            open_file.read_to_string(&mut file_content)?;
            println!("Input file used: {}", path);
        }
        Err(err) => {
            println!("Failed to open `{}` with error: {}", path, err);
            return Ok(());
        }
    }

    match challenge {
        1 => {
            println!("Part 1:");
            challenges::day1::part1(&file_content)?;
            println!("Part 2:");
            challenges::day1::part2(&file_content)?;
        }
        2 => {
            println!("Part 1:");
            challenges::day2::part1(&file_content)?;
            println!("Part 2:");
            challenges::day2::part2(&file_content)?;
        }
        3 => {
            println!("Part 1:");
            challenges::day3::part1(&file_content)?;
            println!("Part 2:");
            challenges::day3::part2(&file_content)?;
        }
        4 => {
            println!("Part 1:");
            challenges::day4::part1(&file_content)?;
            println!("Part 2:");
            challenges::day4::part2(&file_content)?;
        }
        _ => (),
    }

    Ok(())
}

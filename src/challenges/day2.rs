use std::error::Error;

pub fn part1(file_content: &String) -> Result<(), Box<dyn Error>> {
    let lines: Vec<(&str, &str)> = file_content
        .split("\n")
        .map(|e| {
            (
                e.split(" ").collect::<Vec<&str>>()[0],
                e.split(" ").collect::<Vec<&str>>()[1],
            )
        })
        .collect();

    let (mut x, mut y) = (0, 0);
    for cmd in lines {
        match cmd.0 {
            "forward" => x += cmd.1.parse::<u32>()?,
            "down" => y += cmd.1.parse::<u32>()?,
            "up" => y -= cmd.1.parse::<u32>()?,
            _ => (),
        }
    }

    println!("{}", x * y);

    Ok(())
}

pub fn part2(file_content: &String) -> Result<(), Box<dyn Error>> {
    let lines: Vec<(&str, &str)> = file_content
        .split("\n")
        .map(|e| {
            (
                e.split(" ").collect::<Vec<&str>>()[0],
                e.split(" ").collect::<Vec<&str>>()[1],
            )
        })
        .collect();

    let (mut x, mut y, mut aim) = (0, 0, 0);
    for cmd in lines {
        match cmd.0 {
            "forward" => {
                x += cmd.1.parse::<u32>()?;
                y += cmd.1.parse::<u32>()? * aim;
            }
            "down" => aim += cmd.1.parse::<u32>()?,
            "up" => aim -= cmd.1.parse::<u32>()?,
            _ => (),
        }
    }

    println!("{}", x * y);

    Ok(())
}

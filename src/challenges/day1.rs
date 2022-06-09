use std::{error::Error, fs::File, io::Read};

pub fn part1(input_file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut file_content = String::new();
    input_file.read_to_string(&mut file_content)?;

    let lines: Vec<&str> = file_content.split("\n").collect();
    let numbers: Vec<u32> = lines
        .into_iter()
        .map(|e| e.parse::<u32>().unwrap())
        .collect();

    let mut counter = 0;
    for i in 0..numbers.len() - 1 {
        if numbers[i] < numbers[i + 1] {
            counter += 1;
        }
    }

    println!("{}", counter);

    Ok(())
}

pub fn part2(input_file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut file_content = String::new();
    input_file.read_to_string(&mut file_content)?;

    let lines: Vec<&str> = file_content.split("\n").collect();
    let numbers: Vec<u32> = lines
        .into_iter()
        .map(|e| e.parse::<u32>().unwrap())
        .collect();

    let mut counter = 0;
    for i in 0..numbers.len() - 3 {
        if numbers[i] + numbers[i + 1] + numbers[i + 2]
            < numbers[i + 1] + numbers[i + 2] + numbers[i + 3]
        {
            counter += 1;
        }
    }

    println!("{}", counter);

    Ok(())
}

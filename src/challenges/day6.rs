use std::{collections::VecDeque, error::Error};

pub fn part1(file_content: &String) -> Result<(), Box<dyn Error>> {
    let lanternfishes = file_content
        .split(",")
        .map(|e| e.parse::<u32>().unwrap_or_default())
        .collect::<Vec<u32>>();

    let mut lanternfish_freq = VecDeque::from_iter(vec![0; 9]);

    lanternfishes
        .iter()
        .for_each(|e| lanternfish_freq[*e as usize] += 1);

    for _ in 0..80 {
        let dying_lanternfishes = lanternfish_freq[0];

        lanternfish_freq.pop_front();

        lanternfish_freq[6] += dying_lanternfishes;
        lanternfish_freq.push_back(dying_lanternfishes);
    }

    println!("{}", lanternfish_freq.iter().sum::<u128>());

    Ok(())
}

pub fn part2(file_content: &String) -> Result<(), Box<dyn Error>> {
    let lanternfishes = file_content
        .split(",")
        .map(|e| e.parse::<u32>().unwrap_or_default())
        .collect::<Vec<u32>>();

    let mut lanternfish_freq = VecDeque::from_iter(vec![0; 9]);

    lanternfishes
        .iter()
        .for_each(|e| lanternfish_freq[*e as usize] += 1);

    for _ in 0..256 {
        let dying_lanternfishes = lanternfish_freq[0];

        lanternfish_freq.pop_front();

        lanternfish_freq[6] += dying_lanternfishes;
        lanternfish_freq.push_back(dying_lanternfishes);
    }

    println!("{}", lanternfish_freq.iter().sum::<u128>());

    Ok(())
}

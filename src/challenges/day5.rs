use std::{collections::HashMap, error::Error};

pub fn part1(file_content: &String) -> Result<(), Box<dyn Error>> {
    let lines = file_content.split("\n");

    let mut map = HashMap::new();

    for line in lines {
        let coord = line
            .split(" -> ")
            .map(|e| {
                let numbers = e.split(",").collect::<Vec<&str>>();

                (
                    numbers[0].parse::<u32>().unwrap_or_default(),
                    numbers[1].parse::<u32>().unwrap_or_default(),
                )
            })
            .collect::<Vec<(u32, u32)>>();

        let (start, end) = (coord[0], coord[1]);

        if start.0 == end.0 {
            let (s, e) = (start.1.min(end.1), start.1.max(end.1));

            for i in s..e + 1 {
                if let Some(val) = map.get(&(start.0, i)).cloned() {
                    map.insert((start.0, i), val + 1);
                } else {
                    map.insert((start.0, i), 1);
                }
            }
        } else if start.1 == end.1 {
            let (s, e) = (start.0.min(end.0), start.0.max(end.0));

            for i in s..e + 1 {
                if let Some(val) = map.get(&(i, start.1)).cloned() {
                    map.insert((i, start.1), val + 1);
                } else {
                    map.insert((i, start.1), 1);
                }
            }
        }
    }

    println!("{}", map.iter().filter(|e| *e.1 > 1).count());

    Ok(())
}

pub fn part2(file_content: &String) -> Result<(), Box<dyn Error>> {
    let lines = file_content.split("\n");

    let mut map = HashMap::new();

    for line in lines {
        let coord = line
            .split(" -> ")
            .map(|e| {
                let numbers = e.split(",").collect::<Vec<&str>>();

                (
                    numbers[0].parse::<u32>().unwrap_or_default(),
                    numbers[1].parse::<u32>().unwrap_or_default(),
                )
            })
            .collect::<Vec<(u32, u32)>>();

        let (start, end) = (coord[0], coord[1]);
        let delta;

        if start.0 == end.0 {
            delta = (
                0,
                (end.1 as i32 - start.1 as i32) / (end.1 as i32 - start.1 as i32).abs(),
            );
        } else if start.1 == end.1 {
            delta = (
                (end.0 as i32 - start.0 as i32) / (end.0 as i32 - start.0 as i32).abs(),
                0,
            );
        } else if (end.1 as i32 - start.1 as i32).abs() == (end.0 as i32 - start.0 as i32).abs() {
            delta = (
                (end.0 as i32 - start.0 as i32) / (end.0 as i32 - start.0 as i32).abs(),
                (end.1 as i32 - start.1 as i32) / (end.1 as i32 - start.1 as i32).abs(),
            );
        } else {
            continue;
        }

        let mut curr = (start.0, start.1);

        loop {
            if let Some(val) = map.get(&curr).cloned() {
                map.insert(curr, val + 1);
            } else {
                map.insert(curr, 1);
            }

            if curr == end {
                break;
            }

            curr = (
                (curr.0 as i32 + delta.0) as u32,
                (curr.1 as i32 + delta.1) as u32,
            );
        }
    }

    println!("{}", map.iter().filter(|e| *e.1 > 1).count());

    Ok(())
}

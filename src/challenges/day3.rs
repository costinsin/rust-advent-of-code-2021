use std::error::Error;

pub fn part1(file_content: &String) -> Result<(), Box<dyn Error>> {
    let lines: Vec<&str> = file_content.split("\n").collect();

    let string_len = lines[0].len();

    let freq = lines.into_iter().fold(vec![(0, 0); string_len], |acc, e| {
        let mut new_acc = acc;

        for i in 0..string_len {
            if e.as_bytes()[i] == '0' as u8 {
                new_acc[i].0 += 1;
            } else {
                new_acc[i].1 += 1;
            }
        }

        new_acc
    });

    let (gamma, epsilon) = freq
        .clone()
        .into_iter()
        .fold((0 as u32, 0 as u32), |acc, e| {
            let mut new_acc = (acc.0 << 1, acc.1 << 1);

            if e.1 >= e.0 {
                new_acc.0 += 1;
            } else {
                new_acc.1 += 1;
            }

            new_acc
        });

    println!("{}", gamma * epsilon);

    Ok(())
}

pub fn part2(file_content: &String) -> Result<(), Box<dyn Error>> {
    let lines: Vec<&str> = file_content.split("\n").collect();

    let string_len = lines[0].len();

    let mut collection_common = lines.clone();
    let mut collection_least = lines.clone();
    for i in 0..string_len {
        if collection_common.len() > 1 {
            let freq_common = collection_common.iter().fold((0, 0), |acc, e| {
                let mut new_acc = acc;

                if e.as_bytes()[i] == '0' as u8 {
                    new_acc.0 += 1;
                } else {
                    new_acc.1 += 1;
                }

                new_acc
            });

            if freq_common.0 > freq_common.1 {
                collection_common = collection_common
                    .into_iter()
                    .filter(|e| e.as_bytes()[i] == '0' as u8)
                    .collect();
            } else {
                collection_common = collection_common
                    .into_iter()
                    .filter(|e| e.as_bytes()[i] == '1' as u8)
                    .collect();
            }
        }

        if collection_least.len() > 1 {
            let freq_least = collection_least.iter().fold((0, 0), |acc, e| {
                let mut new_acc = acc;

                if e.as_bytes()[i] == '0' as u8 {
                    new_acc.0 += 1;
                } else {
                    new_acc.1 += 1;
                }

                new_acc
            });

            if freq_least.0 > freq_least.1 {
                collection_least = collection_least
                    .into_iter()
                    .filter(|e| e.as_bytes()[i] == '1' as u8)
                    .collect();
            } else {
                collection_least = collection_least
                    .into_iter()
                    .filter(|e| e.as_bytes()[i] == '0' as u8)
                    .collect();
            }
        }

        if collection_least.len() == 1 && collection_common.len() == 1 {
            break;
        }
    }

    let oxygen = isize::from_str_radix(collection_common[0], 2)?;
    let co2 = isize::from_str_radix(collection_least[0], 2)?;
    println!("{}", oxygen * co2);

    Ok(())
}

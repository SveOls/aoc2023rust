use std::{
    collections::HashMap,
    error::Error,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("day14")?;
    println!("day 14a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 14b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut data: Vec<Vec<_>> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == 'O' {
                for k in 1..=i {
                    if data[i - k][j] == '.' {
                        data[i - k][j] = 'O';
                        data[i - k + 1][j] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
    }
    // for i in data {
    //     for j in i {
    //         print!("{} ", j)
    //     }
    //     println!()
    // }
    let res: usize = data
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| x.iter().filter(|c| c == &&'O').count() * (i + 1))
        .sum();
    Ok(res as i64)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut data: Vec<Vec<_>> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    let mut check = HashMap::new();
    let mut rounder = 0;
    let goal = 1000000000;
    while rounder < goal {
        if let Some(a) = check.insert(data.clone(), rounder) {
            // println!("{}", check.len());
            let multiple = rounder - a;
            if rounder + multiple <= goal {
                rounder += multiple
            }
        }

        for round in 0..4 {
            for i in 0..data.len() {
                for j in 0..data[0].len() {
                    let mut i = i;
                    let mut j = j;
                    match round {
                        0 => {}
                        1 => {}
                        2 => i = data.len() - i - 1,
                        3 => j = data[0].len() - j - 1,
                        _ => panic!(),
                    }
                    if data[i][j] == 'O' {
                        match round {
                            0 => {
                                for k in 1..=i {
                                    if data[i - k][j] == '.' {
                                        data[i - k][j] = 'O';
                                        data[i - k + 1][j] = '.';
                                    } else {
                                        break;
                                    }
                                }
                            }
                            1 => {
                                for k in 1..=j {
                                    if data[i][j - k] == '.' {
                                        data[i][j - k] = 'O';
                                        data[i][j - k + 1] = '.';
                                    } else {
                                        break;
                                    }
                                }
                            }
                            2 => {
                                for k in i + 1..data.len() {
                                    if data[k][j] == '.' {
                                        data[k][j] = 'O';
                                        data[k - 1][j] = '.';
                                    } else {
                                        break;
                                    }
                                }
                            }
                            3 => {
                                for k in j + 1..data[0].len() {
                                    if data[i][k] == '.' {
                                        data[i][k] = 'O';
                                        data[i][k - 1] = '.';
                                    } else {
                                        break;
                                    }
                                }
                            }
                            _ => panic!(),
                        }
                    }
                }
            }
        }
        // for i in &data {
        //     for j in i {
        //         print!("{} ", j)
        //     }
        //     println!()
        // }
        rounder += 1;
    }
    let res: usize = data
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| x.iter().filter(|c| c == &&'O').count() * (i + 1))
        .sum();
    Ok(res as i64)
}

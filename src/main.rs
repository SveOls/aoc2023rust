#![feature(iter_array_chunks, iterator_try_collect)]
#![feature(array_try_from_fn)]
#![feature(let_chains)]
#![feature(array_windows)]
#![feature(iter_intersperse)]
#![feature(hash_extract_if)]

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day7;
mod day8;
mod day9;

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::Read,
};

fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::Instant::now();
    day1a();
    day1b();
    day2a();
    day2b();
    day3a();
    day3b();
    day4a();
    day4b();
    day5a();
    day5b();
    day6a();
    day6b();
    day7::parta();
    day7::partb();
    day8::run()?;
    day9::run()?;
    day10::run()?;
    day11::run()?;
    day12::run()?;
    day13::run()?;
    day14::run()?;
    day15::run()?;
    day16::run()?;
    day17::run()?;
    day18::run()?;
    day19::run()?;
    day20::run()?;
    day21::run()?;
    day22::run()?;
    day23::run()?;
    day24::run()?;
    day25::run()?;
    println!("{:?}", now.elapsed());
    Ok(())
}

fn parse(adr: &str) -> Result<String, Box<dyn Error>> {
    let mut contents = String::new();
    File::open(format!("input/2023/{adr}.txt"))?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn day6b() {
    let contents = parse("day6").unwrap();
    let mut res = 1;
    let (time, distance) = contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .array_chunks::<2>()
        .map(|y| {
            (
                y[0].chars()
                    .filter(char::is_ascii_digit)
                    .collect::<String>()
                    .parse::<f64>()
                    .unwrap(),
                y[1].chars()
                    .filter(char::is_ascii_digit)
                    .collect::<String>()
                    .parse::<f64>()
                    .unwrap(),
            )
        })
        .next()
        .unwrap();

    // println!("{time} {distance}");

    let minusb = 0.5 * time;
    let rootbfirac = 0.5 * (time.powi(2) - 4. * distance).sqrt();
    res *= (minusb + rootbfirac).floor() as u32 - (minusb - rootbfirac).ceil() as u32 + 1;

    println!("6b {res}");
}

fn day6a() {
    let contents = parse("day6").unwrap();
    let mut res = 1;
    for (time, distance) in contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .array_chunks::<2>()
        .flat_map(|y| y[0].split_whitespace().zip(y[1].split_whitespace()))
        .skip(1)
    {
        // println!("{time} {distance}");

        let time: f64 = time.parse().unwrap();
        let distance: f64 = distance.parse().unwrap();
        let minusb = 0.5 * time;
        let rootbfirac = 0.5 * (time.powi(2) - 4. * distance).sqrt();
        res *= (minusb + rootbfirac).floor() as u32 - (minusb - rootbfirac).ceil() as u32 + 1
    }
    println!("6a {res}");
}

fn day5b() {
    let contents = parse("day5").unwrap();
    let mut stuff = contents.split('\n');
    let seeds: Vec<usize> = stuff
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    let mut data = Vec::new();
    let mut minidata = Vec::new();
    while let Some(line) = stuff.next() {
        if line.is_empty() {
            if !minidata.is_empty() {
                data.push(minidata.clone());
            }
            minidata = Vec::new();
            stuff.next();
            continue;
        }
        let mut liners = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        minidata.push([
            liners.next().unwrap(),
            liners.next().unwrap(),
            liners.next().unwrap(),
        ]);
    }
    let mut res = 0;
    for i in 0.. {
        let mut temp_seed = i;
        for map in data.iter().rev() {
            for q in map {
                if temp_seed >= q[0] && temp_seed < q[0] + q[2] {
                    temp_seed += q[1];
                    temp_seed -= q[0];
                    break;
                }
            }
        }
        if seeds
            .chunks(2)
            .any(|x| temp_seed > x[0] && temp_seed < x[0] + x[1])
        {
            res = i;
            break;
        }
    }
    println!("5b {:?}", res);
}

fn day5a() {
    let contents = parse("day5").unwrap();
    let mut stuff = contents.split('\n');
    let seeds: Vec<usize> = stuff
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    let mut data = Vec::new();
    let mut minidata = Vec::new();
    while let Some(line) = stuff.next() {
        if line.is_empty() {
            if !minidata.is_empty() {
                data.push(minidata.clone());
            }
            minidata = Vec::new();
            stuff.next();
            continue;
        }
        let mut liners = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        minidata.push([
            liners.next().unwrap(),
            liners.next().unwrap(),
            liners.next().unwrap(),
        ]);
    }
    let mut res = Vec::new();
    for seed in seeds {
        let mut temp_seed = seed;
        for map in &data {
            for q in map {
                // println!("{:?}", q);
                if temp_seed > q[1] && temp_seed < q[1] + q[2] {
                    temp_seed += q[0];
                    temp_seed -= q[1];
                    break;
                }
            }
        }
        res.push(temp_seed);
    }
    println!("5a {:?}", res.iter().min().unwrap());
}

fn day4b() {
    let contents = parse("day4").unwrap();
    let mut cards = vec![1; contents.split('\n').filter(|x| !x.is_empty()).count()];
    for (i, line) in contents.split('\n').filter(|x| !x.is_empty()).enumerate() {
        let mut thing = line.split(|c| c == ':' || c == '|').skip(1);
        let winnings = thing
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<HashSet<u32>>();
        let count = thing
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .filter(|x| winnings.contains(x))
            .count();
        for j in 1..=count {
            cards[i + j] += cards[i]
        }
    }
    println!("4b {}", cards.iter().sum::<u32>());
}

fn day4a() {
    let contents = parse("day4").unwrap();
    let mut res = 0;
    for line in contents.split('\n').filter(|x| !x.is_empty()) {
        let mut thing = line.split(|c| c == ':' || c == '|').skip(1);
        let winnings = thing
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<HashSet<u32>>();
        let count = thing
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .filter(|x| winnings.contains(x))
            .count();
        if count != 0 {
            res += 2usize.pow(count as u32 - 1)
        }
    }
    println!("4a {res}")
}

fn day3b() {
    let contents = parse("day3").unwrap();
    let data: Vec<Vec<_>> = contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    let mut gears: HashMap<[usize; 2], Vec<u32>> = HashMap::new();
    for i in 0..data.len() {
        let mut j = 0;
        let mut number: Option<u32> = None;
        loop {
            if let Some(a) = data[i].get(j).and_then(|x| x.to_digit(10)) {
                let x = number.get_or_insert(0);
                *x *= 10;
                *x += a;
            } else if let Some(a) = number.take() {
                if let Some(pp) = data
                    .iter()
                    .enumerate()
                    .take(i + 2)
                    .skip(i.saturating_sub(1))
                    .flat_map(|x| {
                        x.1.iter()
                            .enumerate()
                            .take(j + 1)
                            .skip(
                                j.saturating_sub(2 + a.checked_ilog10().unwrap() as usize)
                                    ,
                            )
                            .map(move |o| (x.0, o.0, *o.1))
                    })
                    .find(|x| x.2 == '*')
                {
                    let b = gears.entry([pp.0, pp.1]).or_default();
                    b.push(a);
                } else {
                }
            }
            if j >= data[i].len() {
                break;
            } else {
                j += 1;
            }
        }
    }
    println!(
        "3b {}",
        gears
            .values()
            .filter(|x| x.len() == 2)
            .map(|x| x[0] * x[1])
            .sum::<u32>()
    );
}

fn day3a() {
    let contents = parse("day3").unwrap();
    let data: Vec<Vec<_>> = contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    let mut res = 0;
    for i in 0..data.len() {
        let mut j = 0;
        let mut number: Option<u32> = None;
        loop {
            if let Some(a) = data[i].get(j).and_then(|x| x.to_digit(10)) {
                let x = number.get_or_insert(0);
                *x *= 10;
                *x += a;
            } else if let Some(a) = number.take() {
                if data[i.saturating_sub(1)..(i + 2).min(data.len())]
                    .iter()
                    .flat_map(|x| {
                        &x[j.saturating_sub(2 + a.checked_ilog10().unwrap() as usize)
                            ..(j + 1).min(data[i].len())]
                    })
                    .any(|&x| x != '.' && !x.is_ascii_digit())
                {
                    res += a;
                }
            }
            if j >= data[i].len() {
                break;
            } else {
                j += 1;
            }
        }
    }
    println!("3a {}", res);
}

fn day2a() {
    let contents = parse("day2").unwrap();
    let maxex = [12, 13, 14];
    let mut res = 0;
    for game in contents.split('\n').filter(|x| !x.is_empty()) {
        let mut gaem = game.split(':');
        let num = gaem
            .next()
            .map(|x| {
                x.chars()
                    .filter(|x| x.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
            })
            .unwrap()
            .unwrap();
        let mut max = [0, 0, 0];
        for (col, am) in gaem
            .next()
            .unwrap()
            .split(|c| c == ';' || c == ',')
            .map(|y| {
                (
                    y.chars()
                        .filter(|t| t.is_ascii_alphabetic())
                        .collect::<String>(),
                    y.chars()
                        .filter(|t| t.is_ascii_digit())
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap(),
                )
            })
        {
            match col.as_str() {
                "red" => max[0] = max[0].max(am),
                "green" => max[1] = max[1].max(am),
                "blue" => max[2] = max[2].max(am),
                _ => panic!(),
            }
        }
        if maxex.iter().zip(max.iter()).all(|x| x.0 >= x.1) {
            res += num;
        }
    }
    println!("2a {}", res);
}

fn day2b() {
    let contents = parse("day2").unwrap();
    let mut res = 0;
    for game in contents.split('\n').filter(|x| !x.is_empty()) {
        let mut gaem = game.split(':');
        let _num = gaem
            .next()
            .map(|x| {
                x.chars()
                    .filter(|x| x.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
            })
            .unwrap()
            .unwrap();
        let mut max = [1, 1, 1];
        for (col, am) in gaem
            .next()
            .unwrap()
            .split(|c| c == ';' || c == ',')
            .map(|y| {
                (
                    y.chars()
                        .filter(|t| t.is_ascii_alphabetic())
                        .collect::<String>(),
                    y.chars()
                        .filter(|t| t.is_ascii_digit())
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap(),
                )
            })
        {
            match col.as_str() {
                "red" => max[0] = max[0].max(am),
                "green" => max[1] = max[1].max(am),
                "blue" => max[2] = max[2].max(am),
                _ => panic!(),
            }
        }
        res += max.iter().product::<u32>();
    }
    println!("2b {}", res);
}

fn day1a() {
    let contents = parse("day1").unwrap();
    let res: u32 = contents
        .split('\n')
        .map(|x| {
            x.chars()
                .filter_map(|y| y.to_digit(10))
                .flat_map(|l| std::iter::repeat(l).take(2))
        })
        .map(|mut z| z.next().unwrap_or(0) * 10 + z.last().unwrap_or(0))
        .sum();
    println!("1a {res}");
}

fn day1b() {
    let contents = parse("day1").unwrap();
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut res = 0;
    for line in contents.split('\n').filter(|x| !x.is_empty()) {
        let mut first = None;
        let mut last = None;
        for i in 0..line.chars().count() {
            if let Some(a) = line.chars().nth(i).unwrap().to_digit(10) {
                first = first.or(Some(a));
                last = Some(a);
            } else if let Some(b) = words.iter().position(|x| {
                x.chars()
                    .zip(line.chars().skip(i).chain(std::iter::repeat('!')))
                    .all(|x| x.0 == x.1)
            }) {
                first = first.or(Some(b as u32 + 1));
                last = Some(b as u32 + 1);
            }
        }
        res += first.unwrap() * 10 + last.unwrap();
    }
    println!("1b {res}");
}

use std::{
    collections::HashMap,
    error::Error,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("day8")?;
    println!("day 8a: {}", parta(&file)?);
    println!("day 8b: {}", partb(&file)?);
    Ok(())
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut input = inp.split('\n').filter(|x| !x.is_empty());
    let orient: Vec<char> = input.next().unwrap().chars().collect();
    let mut data: HashMap<[char; 3], [[char; 3]; 2]> = HashMap::new();
    let mut currentpos = ['A'; 3];
    for a in input {
        let mut things = a
            .split(|x: char| !x.is_alphanumeric())
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().collect::<Vec<_>>().try_into().unwrap());
        data.insert(
            things.next().unwrap(),
            [things.next().unwrap(), things.next().unwrap()],
        );
    }
    let mut res = 0;
    while currentpos != ['Z'; 3] {
        match orient[res % orient.len()] {
            'R' => currentpos = data.get(&currentpos).unwrap()[1],
            'L' => currentpos = data.get(&currentpos).unwrap()[0],
            _ => unreachable!(),
        }
        res += 1;
    }

    Ok(res as i64)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut input = inp.split('\n').filter(|x| !x.is_empty());
    let orient: Vec<char> = input.next().unwrap().chars().collect();
    let mut data: HashMap<[char; 3], [[char; 3]; 2]> = HashMap::new();
    for a in input {
        let mut things = a
            .split(|x: char| !x.is_ascii_alphanumeric())
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().collect::<Vec<_>>().try_into().unwrap());
        data.insert(
            things.next().unwrap(),
            [things.next().unwrap(), things.next().unwrap()],
        );
    }
    let candidates: Vec<_> = data.keys().filter(|x| x[2] == 'A').collect();
    let mut info: Vec<(Vec<usize>, [usize; 2])> = Vec::new();
    for candidate in candidates {
        let mut res = 0;
        let mut currentpos = candidate;
        let mut opportunities = Vec::new();
        let mut unvisited: HashMap<(&[char; 3], usize), usize> = HashMap::new();
        loop {
            if let Some(a) = unvisited.insert((&currentpos, res % orient.len()), res) {
                // opportunities = list of times goals are passed
                // array = start of loop, length of loop
                info.push((opportunities, [a, res - a]));
                break;
            } else if currentpos[2] == 'Z' {
                opportunities.push(res);
            }
            match orient[res % orient.len()] {
                'R' => currentpos = &data.get(currentpos).unwrap()[1],
                'L' => currentpos = &data.get(currentpos).unwrap()[0],
                _ => unreachable!(),
            }
            res += 1;
        }
    }
    Ok(info.into_iter().map(|x| x.0[0]).fold(1, num::integer::lcm) as i64)
}

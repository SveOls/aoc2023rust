use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("adventofcode.com_2023_day_12_input.txt")?;
    println!("day 12a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 12b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut a = inp.split('\n').filter(|x| !x.is_empty());
    let mut dater = VecDeque::new();
    for b in a.by_ref() {
        let streng: Vec<_> = b.split_whitespace().next().unwrap().chars().collect();
        let kriter: Vec<u32> = b
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|x| x.parse())
            .try_collect()?;
        // let adds = kriter.iter().sum::<u32>() - streng.iter().filter(|&&x| x == '#').count() as u32;

        dater.push_back((streng, kriter));
    }
    let mut res = 0;
    'outer: while let Some(thing) = dater.pop_front() {
        if let Some(a) = thing.0.iter().position(|&x| x == '?') {
            if thing.1.iter().sum::<u32>() - thing.0.iter().filter(|&&x| x == '#').count() as u32
                > 0
            {
                let mut temp = thing.0.clone();
                temp[a] = '#';
                dater.push_back((temp, thing.1.clone()))
            }
            if thing.0.iter().filter(|&&x| x == '#' || x == '?').count() as u32
                > thing.1.iter().sum::<u32>()
            {
                let mut temp = thing.0.clone();
                temp[a] = '.';
                dater.push_back((temp, thing.1.clone()))
            }
        } else {
            let mut in_row = 0;
            let mut index = 0;
            for c in thing.0.iter().chain(std::iter::once(&'.')) {
                match c {
                    '#' => in_row += 1,
                    _ => {
                        if in_row > 0 {
                            if in_row != thing.1[index] {
                                continue 'outer;
                            } else {
                                index += 1;
                            }
                        }
                        in_row = 0;
                    }
                }
            }
            // println!("{:?}", thing);
            res += 1;
        }
    }
    Ok(res)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut a = inp.split('\n').filter(|x| !x.is_empty());
    let mut dater: VecDeque<((Vec<char>, Vec<u32>), Option<usize>)> = VecDeque::new();
    for b in a.by_ref() {
        let mut streng: Vec<_> = b.split_whitespace().next().unwrap().chars().collect();
        let mut kriter: Vec<u32> = b
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|x| x.parse())
            .try_collect()?;
        // let adds = kriter.iter().sum::<u32>() - streng.iter().filter(|&&x| x == '#').count() as u32;
        let krithom = kriter.clone();
        let strhom = streng.clone();
        kriter.append(&mut krithom.clone());
        kriter.append(&mut krithom.clone());
        kriter.append(&mut krithom.clone());
        kriter.append(&mut krithom.clone());
        streng.push('?');
        streng.append(&mut strhom.clone());
        streng.push('?');
        streng.append(&mut strhom.clone());
        streng.push('?');
        streng.append(&mut strhom.clone());
        streng.push('?');
        streng.append(&mut strhom.clone());
        dater.push_back(((streng, kriter), None));
    }
    let mut res = 0;
    // let mut eeer = HashSet::new();
    // let mut now = std::time::Instant::now();

    let mut memo: HashMap<(Vec<char>, Vec<u32>), usize> = HashMap::new();
    let mut test = 0;
    let mut last = Vec::new();
    while let Some((thing, vale)) = dater.pop_front() {
        if last != thing.1 {
            memo.clear();
            last = thing.1.clone();
        }
        // println!("{:?} {:?}", thing, vale);
        if vale.is_none() {
            test += 1;
        }
        if test % 100000 == 0 {
            // println!("{}", visited.len())
            // println!("{:?}", thing);
            // println!("{:?}", dater.front());
            // println!("{:?}", memo.len());
        }
        if thing.0.iter().any(|&c| c == '?') && vale.is_none() {
            dater.push_front((thing.clone(), Some(res)))
        }
        if let Some(val) = vale {
            // println!("{:?}", old_thing.0);
            let quest_pos = std::iter::once(&'.')
                .chain(thing.0.iter())
                // .inspect(|x| println!("{x}"))
                .enumerate()
                .take_while(|&c| c.1 != &'?')
                .filter(|&x| x.1 == &'.')
                .last()
                .unwrap()
                .0;

            let total_locked = thing
                .0
                .iter()
                .take(quest_pos)
                .filter(|&&c| c == '#')
                .count() as u32;
            let posue = thing
                .1
                .iter()
                .chain(std::iter::once(&0))
                .scan(0, |acc, x| {
                    *acc += x;
                    Some(*acc - x)
                })
                .position(|x| x == total_locked)
                .unwrap();
            // println!(
            //     "{:?} {} - {} = {}, {} and {}, {}",
            //     thing.0,
            //     res,
            //     val,
            //     res - val,
            //     quest_pos,
            //     total_locked,
            //     posue
            // );
            let oie = (thing.0[quest_pos..].into(), thing.1[posue..].into());
            // println!("{:?}\n{:?}", old_thing.0, old_thing.1);
            // println!("{:?}\n{:?}", oie.0, oie.1);
            memo.insert(oie, res - val);
            continue;
        }
        if let Some(quest_pos) = std::iter::once(&'.')
            .chain(thing.0.iter())
            .enumerate()
            .take_while(|&c| c.1 != &'?')
            .filter(|&x| x.1 == &'.')
            .last()
            .map(|x| x.0)
            && thing.0.iter().any(|c| c == &'?')
        {
            // if Some(&'.') == thing.0.iter().take_while(|&&c| c != '?').last()
            //     && thing.0.iter().any(|&c| c == '?')
            // {
            let total_locked = thing
                .0
                .iter()
                .take(quest_pos)
                .filter(|&&c| c == '#')
                .count() as u32;
            let posue = thing
                .1
                .iter()
                .chain(std::iter::once(&0))
                .scan(0, |acc, x| {
                    *acc += x;
                    Some(*acc - x)
                })
                .position(|x| x == total_locked)
                .unwrap();
            if let Some(saved) = memo.get(&(thing.0[quest_pos..].into(), thing.1[posue..].into())) {
                // println!("{:?} -> {}", thing, saved);
                res += saved;
                continue;
            }
            // }
            let actual_questpos = thing.0.iter().position(|c| c == &'?').unwrap();
            if thing.1.iter().sum::<u32>() - thing.0.iter().filter(|&&x| x == '#').count() as u32
                > 0
            {
                let mut temp = thing.0.clone();
                temp[actual_questpos] = '#';
                if verify(&temp, &thing.1) {
                    dater.push_front(((temp, thing.1.clone()), None));
                }
            }
            if thing.0.iter().filter(|&&x| x == '#' || x == '?').count() as u32
                > thing.1.iter().sum::<u32>()
            {
                let mut temp = thing.0;
                temp[actual_questpos] = '.';
                if verify(&temp, &thing.1) {
                    dater.push_front(((temp, thing.1), None));
                }
            }
        } else {
            if verify(&thing.0, &thing.1) {
                res += 1;
            }
        }
    }
    println!("{}, {}", test, memo.len());
    // println!("{}, {}", test, visited.len());
    // for i in memo {
    //     println!("{:?}", i);
    // }
    Ok(res as i64)
}

fn verify(sauna: &[char], pattern: &[u32]) -> bool {
    let mut in_row = 0;
    let mut index = 0;
    for c in sauna.iter().chain(std::iter::once(&'.')) {
        match c {
            '#' => in_row += 1,
            _ => {
                if in_row > 0 {
                    if in_row > pattern[index] {
                        return false;
                    } else if in_row < pattern[index] && c != &'?' {
                        return false;
                    } else {
                        index += 1;
                    }
                }
                in_row = 0;
                if *c == '?' {
                    break;
                }
            }
        }
    }
    true
}

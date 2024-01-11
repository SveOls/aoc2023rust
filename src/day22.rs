use std::{
    collections::HashSet,
    error::Error,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("day22")?;
    println!("day 22a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 22b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut blocks: Vec<[[usize; 3]; 2]> = Vec::new();
    for line in inp.split('\n').filter(|x| !x.is_empty()) {
        let (start, end) = line.split_once('~').unwrap();
        let start = start
            .split(',')
            .map(|x| x.parse())
            .try_collect::<Vec<usize>>()?;
        let end = end
            .split(',')
            .map(|x| x.parse())
            .try_collect::<Vec<usize>>()?;
        blocks.push([
            [
                start[0].min(end[0]),
                start[1].min(end[1]),
                start[2].min(end[2]),
            ],
            [
                start[0].max(end[0]),
                start[1].max(end[1]),
                start[2].max(end[2]),
            ],
        ]);
    }
    // dbg!(&blocks);
    let mut exit_condition = true;
    let mut supported_by = vec![HashSet::new(); blocks.len()];
    while exit_condition {
        exit_condition = false;
        supported_by.iter_mut().for_each(|x| x.clear());
        for i in 0..blocks.len() {
            for z in (1..blocks[i][0][2]).rev() {
                let mut dropper = false;
                for j in 0..blocks.len() {
                    // xcon and ycon are true when theres overlap
                    let xcon = blocks[i][0][0]
                        == blocks[i][0][0].clamp(blocks[j][0][0], blocks[j][1][0])
                        || blocks[i][1][0]
                            == blocks[i][1][0].clamp(blocks[j][0][0], blocks[j][1][0])
                        || blocks[j][0][0]
                            == blocks[j][0][0].clamp(blocks[i][0][0], blocks[i][1][0])
                        || blocks[j][1][0]
                            == blocks[j][1][0].clamp(blocks[i][0][0], blocks[i][1][0]);
                    let ycon = blocks[i][0][1]
                        == blocks[i][0][1].clamp(blocks[j][0][1], blocks[j][1][1])
                        || blocks[i][1][1]
                            == blocks[i][1][1].clamp(blocks[j][0][1], blocks[j][1][1])
                        || blocks[j][0][1]
                            == blocks[j][0][1].clamp(blocks[i][0][1], blocks[i][1][1])
                        || blocks[j][1][1]
                            == blocks[j][1][1].clamp(blocks[i][0][1], blocks[i][1][1]);

                    if (z == blocks[j][0][2] || z == blocks[j][1][2]) && xcon && ycon {
                        dropper = true;
                        supported_by[i].insert(j);
                    }
                    // if i == 1 && j == 0 {
                    //     dbg!(xcon);
                    //     dbg!(ycon);
                    //     panic!()
                    // }
                }
                if dropper {
                    break;
                }
                blocks[i][0][2] -= 1;
                blocks[i][1][2] -= 1;
                exit_condition = true;
            }
        }
    }
    // dbg!(&blocks);
    // dbg!(&supported_by);
    let res = blocks.len()
        - supported_by
            .iter()
            .filter(|x| x.len() == 1)
            .flat_map(|x| x.iter())
            .collect::<HashSet<_>>()
            .len();
    // dbg!(res);
    // dbg!(blocks.len());
    Ok(res as i64)
}

// aabb
// min(a) < max(b)
// max(a) < min(b)

// abab
// min(a) < max(b)
// max(a) > min(b)

// bbaa
// min(a) > max(b)
// max(a) > min(b)

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut blocks: Vec<[[usize; 3]; 2]> = Vec::new();
    for line in inp.split('\n').filter(|x| !x.is_empty()) {
        let (start, end) = line.split_once('~').unwrap();
        let start = start
            .split(',')
            .map(|x| x.parse())
            .try_collect::<Vec<usize>>()?;
        let end = end
            .split(',')
            .map(|x| x.parse())
            .try_collect::<Vec<usize>>()?;
        blocks.push([
            [
                start[0].min(end[0]),
                start[1].min(end[1]),
                start[2].min(end[2]),
            ],
            [
                start[0].max(end[0]),
                start[1].max(end[1]),
                start[2].max(end[2]),
            ],
        ]);
    }
    // dbg!(&blocks);
    let mut exit_condition = true;
    let mut supported_by = vec![HashSet::new(); blocks.len()];
    while exit_condition {
        exit_condition = false;
        supported_by.iter_mut().for_each(|x| x.clear());
        for i in 0..blocks.len() {
            for z in (1..blocks[i][0][2]).rev() {
                let mut dropper = false;
                for j in 0..blocks.len() {
                    // xcon and ycon are true when theres overlap
                    let xcon = blocks[i][0][0]
                        == blocks[i][0][0].clamp(blocks[j][0][0], blocks[j][1][0])
                        || blocks[i][1][0]
                            == blocks[i][1][0].clamp(blocks[j][0][0], blocks[j][1][0])
                        || blocks[j][0][0]
                            == blocks[j][0][0].clamp(blocks[i][0][0], blocks[i][1][0])
                        || blocks[j][1][0]
                            == blocks[j][1][0].clamp(blocks[i][0][0], blocks[i][1][0]);
                    let ycon = blocks[i][0][1]
                        == blocks[i][0][1].clamp(blocks[j][0][1], blocks[j][1][1])
                        || blocks[i][1][1]
                            == blocks[i][1][1].clamp(blocks[j][0][1], blocks[j][1][1])
                        || blocks[j][0][1]
                            == blocks[j][0][1].clamp(blocks[i][0][1], blocks[i][1][1])
                        || blocks[j][1][1]
                            == blocks[j][1][1].clamp(blocks[i][0][1], blocks[i][1][1]);

                    if (z == blocks[j][0][2] || z == blocks[j][1][2]) && xcon && ycon {
                        dropper = true;
                        supported_by[i].insert(j);
                    }
                    // if i == 1 && j == 0 {
                    //     dbg!(xcon);
                    //     dbg!(ycon);
                    //     panic!()
                    // }
                }
                if dropper {
                    break;
                }
                blocks[i][0][2] -= 1;
                blocks[i][1][2] -= 1;
                exit_condition = true;
            }
        }
    }
    // dbg!(&blocks);
    // dbg!(&supported_by);
    let mut supporting = vec![HashSet::new(); supported_by.len()];
    let mut res = 0;
    for _ in 0.. {
        for (i, x) in supporting.iter_mut().enumerate().take(supported_by.len()) {
            for (j, y) in supported_by.iter().enumerate() {
                if i == j {
                    continue;
                }
                if y.is_empty() {
                    continue;
                }
                if y.iter().all(|&t| {
                    std::iter::once(i)
                        .chain(x.iter().copied())
                        .any(|y| t == y)
                }) {
                    x.insert(j);
                }
            }
        }
        if res == supporting.iter().flat_map(|x| x.iter()).count() {
            break;
        }
        res = supporting.iter().flat_map(|x| x.iter()).count();
    }
    // dbg!(&supporting);
    // // let mut res = 0;
    // // dbg!(res);
    // dbg!(&supported_by);
    // dbg!(blocks.len());
    Ok(res as i64)
}

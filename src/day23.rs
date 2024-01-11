use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("adventofcode.com_2023_day_23_input.txt")?;
    println!("day 23a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 23b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Terrain {
    Path,
    Forest,
    NorthSlope,
    EastSlope,
    SouthSlope,
    WestSlope,
    Tinter,
    Cross,
    Dead,
}

impl From<char> for Terrain {
    fn from(value: char) -> Self {
        use Terrain::*;
        match value {
            '.' => Path,
            '#' => Forest,
            '^' => NorthSlope,
            '>' => EastSlope,
            'v' => SouthSlope,
            '<' => WestSlope,
            'T' => Tinter,
            '+' => Cross,
            '*' => Dead,
            _ => panic!(),
        }
    }
}

impl From<Terrain> for char {
    fn from(rhs: Terrain) -> char {
        use Terrain::*;
        match rhs {
            Path => '.',
            Forest => '#',
            NorthSlope => '^',
            EastSlope => '>',
            SouthSlope => 'v',
            WestSlope => '<',
            Tinter => 'T',
            Cross => '+',
            Dead => '*',
        }
    }
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let data: Vec<Vec<Terrain>> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|c| c.into()).collect())
        .collect();

    let start_pos = [0, data[0].iter().position(|&x| x == Terrain::Path).unwrap()];
    let end_pos = [
        data.len() - 1,
        data[data.len() - 1]
            .iter()
            .position(|&x| x == Terrain::Path)
            .unwrap(),
    ];

    let mut dead_ends = vec![start_pos, end_pos];
    let mut intersections = Vec::new();
    let mut fourway = Vec::new();
    for i in 1..data.len() - 1 {
        for j in 1..data[i].len() - 1 {
            if data[i][j] == Terrain::Forest {
                continue;
            }
            let mut candidates = Vec::new();
            if data[i][j + 1] != Terrain::Forest {
                candidates.push([i, j + 1]);
            }
            if data[i][j - 1] != Terrain::Forest {
                candidates.push([i, j - 1]);
            }
            if data[i + 1][j] != Terrain::Forest {
                candidates.push([i + 1, j]);
            }
            if data[i - 1][j] != Terrain::Forest {
                candidates.push([i - 1, j]);
            }
            match candidates.len() {
                1 => dead_ends.push([i, j]),
                3 => intersections.push([i, j]),
                4 => fourway.push([i, j]),
                _ => {}
            }
        }
    }
    // for i in 0..4 {
    //     let mut new_pos = [((i + 1) % 4) / 2 - ((i + 2) % 4) / 2, ((i + 1) % 4) / 2 - (i % 4) / 2];
    //     dbg!(new_pos);
    // }
    // panic!();
    let mut all = Vec::new();
    all.extend(dead_ends.iter().copied());
    all.extend(intersections.iter().copied());
    all.extend(fourway.iter().copied());

    let mut paths: HashMap<[usize; 2], Vec<([usize; 2], usize)>> = HashMap::new();

    for &pos in all.iter() {
        'outer: for i in 0..4 {
            let mut visited = HashSet::new();
            let mut new_pos = [
                pos[0] + ((i + 1) % 4) / 2 - ((i + 2) % 4) / 2,
                pos[1] + ((i + 1) % 4) / 2 - (i % 4) / 2,
            ];
            if new_pos[0] >= data.len() || new_pos[1] >= data.len() {
                continue;
            }
            let mut i = i;
            visited.insert(pos);
            //
            while !all.contains(&new_pos) {
                visited.insert(new_pos);
                match (i, data[new_pos[0]][new_pos[1]]) {
                    (_, Terrain::Forest) => continue 'outer,
                    (0, Terrain::SouthSlope) => continue 'outer,
                    (1, Terrain::WestSlope) => continue 'outer,
                    (2, Terrain::NorthSlope) => continue 'outer,
                    (3, Terrain::EastSlope) => continue 'outer,
                    (_, Terrain::SouthSlope) => {
                        i = 2;
                        new_pos[0] += 1;
                    }
                    (_, Terrain::WestSlope) => {
                        i = 3;
                        new_pos[1] -= 1;
                    }
                    (_, Terrain::NorthSlope) => {
                        i = 0;
                        new_pos[0] -= 1;
                    }
                    (_, Terrain::EastSlope) => {
                        i = 1;
                        new_pos[1] += 1;
                    }
                    (_, Terrain::Path) => {
                        if data[new_pos[0] - 1][new_pos[1]] != Terrain::Forest
                            && !visited.contains(&[new_pos[0] - 1, new_pos[1]])
                        {
                            i = 0;
                            new_pos[0] -= 1;
                        } else if data[new_pos[0]][new_pos[1] + 1] != Terrain::Forest
                            && !visited.contains(&[new_pos[0], new_pos[1] + 1])
                        {
                            i = 1;
                            new_pos[1] += 1;
                        } else if data[new_pos[0] + 1][new_pos[1]] != Terrain::Forest
                            && !visited.contains(&[new_pos[0] + 1, new_pos[1]])
                        {
                            i = 2;
                            new_pos[0] += 1;
                        } else if data[new_pos[0]][new_pos[1] - 1] != Terrain::Forest
                            && !visited.contains(&[new_pos[0], new_pos[1] - 1])
                        {
                            i = 3;
                            new_pos[1] -= 1;
                        } else {
                            panic!();
                        }
                    }
                    _ => panic!(),
                };
            }
            paths.entry(pos).or_default().push((new_pos, visited.len()));
        }
    }
    Ok(seek_solution(start_pos, &mut paths, end_pos)
        .into_iter()
        .max()
        .unwrap() as i64)
}

// add memoisation for speedup
fn seek_solution(
    pos: [usize; 2],
    data: &mut HashMap<[usize; 2], Vec<([usize; 2], usize)>>,
    end: [usize; 2],
) -> Vec<usize> {
    let mut ret = Vec::new();
    if let Some(info) = data.remove_entry(&pos) {
        for &(future, vals) in &info.1 {
            if future == end {
                ret.push(vals)
            } else {
                ret.extend(
                    seek_solution(future, data, end)
                        .into_iter()
                        .map(|x| x + vals),
                );
            }
        }
        data.insert(info.0, info.1);
    }
    ret
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut data: Vec<Vec<Terrain>> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|c| c.into()).collect())
        .collect();

    data.iter_mut().flat_map(|x| x.iter_mut()).for_each(|x| {
        if *x != Terrain::Forest {
            *x = Terrain::Path
        }
    });

    let start_pos = [0, data[0].iter().position(|&x| x == Terrain::Path).unwrap()];
    let end_pos = [
        data.len() - 1,
        data[data.len() - 1]
            .iter()
            .position(|&x| x == Terrain::Path)
            .unwrap(),
    ];

    let mut dead_ends = vec![start_pos, end_pos];
    let mut intersections = Vec::new();
    let mut fourway = Vec::new();
    for i in 1..data.len() - 1 {
        for j in 1..data[i].len() - 1 {
            if data[i][j] == Terrain::Forest {
                continue;
            }
            let mut candidates = Vec::new();
            if data[i][j + 1] != Terrain::Forest {
                candidates.push([i, j + 1]);
            }
            if data[i][j - 1] != Terrain::Forest {
                candidates.push([i, j - 1]);
            }
            if data[i + 1][j] != Terrain::Forest {
                candidates.push([i + 1, j]);
            }
            if data[i - 1][j] != Terrain::Forest {
                candidates.push([i - 1, j]);
            }
            match candidates.len() {
                1 => dead_ends.push([i, j]),
                3 => intersections.push([i, j]),
                4 => fourway.push([i, j]),
                _ => {}
            }
        }
    }
    let mut all = Vec::new();
    all.extend(dead_ends.iter().copied());
    all.extend(intersections.iter().copied());
    all.extend(fourway.iter().copied());

    let mut paths: HashMap<[usize; 2], Vec<([usize; 2], usize)>> = HashMap::new();

    for &pos in all.iter() {
        'outer: for i in 0..4 {
            let mut visited = HashSet::new();
            let mut new_pos = [
                pos[0] + ((i + 1) % 4) / 2 - ((i + 2) % 4) / 2,
                pos[1] + ((i + 1) % 4) / 2 - (i % 4) / 2,
            ];
            if new_pos[0] >= data.len() || new_pos[1] >= data.len() {
                continue;
            }
            let mut i = i;
            visited.insert(pos);
            //
            while !all.contains(&new_pos) {
                visited.insert(new_pos);
                match (i, data[new_pos[0]][new_pos[1]]) {
                    (_, Terrain::Forest) => continue 'outer,
                    (0, Terrain::SouthSlope) => continue 'outer,
                    (1, Terrain::WestSlope) => continue 'outer,
                    (2, Terrain::NorthSlope) => continue 'outer,
                    (3, Terrain::EastSlope) => continue 'outer,
                    (_, Terrain::SouthSlope) => {
                        i = 2;
                        new_pos[0] += 1;
                    }
                    (_, Terrain::WestSlope) => {
                        i = 3;
                        new_pos[1] -= 1;
                    }
                    (_, Terrain::NorthSlope) => {
                        i = 0;
                        new_pos[0] -= 1;
                    }
                    (_, Terrain::EastSlope) => {
                        i = 1;
                        new_pos[1] += 1;
                    }
                    (_, Terrain::Path) => {
                        if data[new_pos[0] - 1][new_pos[1]] != Terrain::Forest
                            && !visited.contains(&[new_pos[0] - 1, new_pos[1]])
                        {
                            i = 0;
                            new_pos[0] -= 1;
                        } else if data[new_pos[0]][new_pos[1] + 1] != Terrain::Forest
                            && !visited.contains(&[new_pos[0], new_pos[1] + 1])
                        {
                            i = 1;
                            new_pos[1] += 1;
                        } else if data[new_pos[0] + 1][new_pos[1]] != Terrain::Forest
                            && !visited.contains(&[new_pos[0] + 1, new_pos[1]])
                        {
                            i = 2;
                            new_pos[0] += 1;
                        } else if data[new_pos[0]][new_pos[1] - 1] != Terrain::Forest
                            && !visited.contains(&[new_pos[0], new_pos[1] - 1])
                        {
                            i = 3;
                            new_pos[1] -= 1;
                        } else {
                            panic!();
                        }
                    }
                    _ => panic!(),
                };
            }
            paths.entry(pos).or_default().push((new_pos, visited.len()));
        }
    }
    Ok(seek_solution(start_pos, &mut paths, end_pos)
        .into_iter()
        .max()
        .unwrap() as i64)
}

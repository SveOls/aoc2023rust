use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("day21")?;
    println!("day 21a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 21b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let data: Vec<Vec<bool>> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|c| c != '#').collect())
        .collect();
    let start = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .enumerate()
        .find_map(|(i, x)| x.chars().position(|c| c == 'S').map(|y| [i, y]))
        .unwrap();
    let mut maper = HashMap::new();
    maper.insert(start, 1);
    for _ in 0..64 {
        for (key, val) in std::mem::take(&mut maper).into_iter() {
            if let Some(x) = key[0].checked_sub(1) {
                if data[x][key[1]] {
                    *maper.entry([x, key[1]]).or_insert(0) += val;
                }
            }
            if let Some(y) = key[1].checked_sub(1) {
                if data[key[0]][y] {
                    *maper.entry([key[0], y]).or_insert(0) += val;
                }
            }
            if Some(true) == data.get(key[0] + 1).map(|x| x[key[1]]) {
                *maper.entry([key[0] + 1, key[1]]).or_insert(0) += val;
            }
            if Some(true) == data[key[0]].get(key[1] + 1).copied() {
                *maper.entry([key[0], key[1] + 1]).or_insert(0) += val;
            }
        }
    }
    // for i in 0..131 {
    //     for j in 0..131 {
    //         if maper.contains_key(&[i, j]) {
    //             print!("O")
    //         } else if !data[i][j] {
    //             print!("#")
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    Ok(maper.len() as i64)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let data: Vec<Vec<bool>> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|c| c != '#').collect())
        .collect();
    // for i in holes {
    //     data[i[0]][i[1]] = false;
    // }
    let start = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .enumerate()
        .find_map(|(i, x)| x.chars().position(|c| c == 'S').map(|y| [i, y]))
        .unwrap();
    let total_steps = 26501365;
    let h = data.len();
    let center = countr(&data, [65, 65], 66);
    let west = countr(&data, [65, 0], 131);
    let east = countr(&data, [65, 130], 131);
    let north = countr(&data, [0, 65], 131);
    let south = countr(&data, [130, 65], 131);
    let northeast = countr(&data, [0, 130], 65);
    let southeast = countr(&data, [130, 130], 65);
    let northwest = countr(&data, [0, 0], 65);
    let southwest = countr(&data, [130, 0], 65);
    // dbg!(center);
    // dbg!(west);
    // dbg!(east);
    // dbg!(north);
    // dbg!(center[2] + northwest[2] + northeast[2] + southwest[2] + southeast[2]);
    // dbg!(data.iter().flat_map(|x| x.iter()).filter(|&&x| x).count());
    // panic!();

    let center_d = (total_steps - start[0]) / h;
    let even_maps_in_grid = ((center_d) / 2 * 2).pow(2);
    let odd_maps_in_grid = ((center_d - 1) / 2 * 2 + 1).pow(2);
    let mut ret = 0;
    ret += north[1];
    ret += south[1];
    ret += west[1];
    ret += east[1];
    ret += center_d * southwest[1];
    ret += center_d * southeast[1];
    ret += center_d * northwest[1];
    ret += center_d * northeast[1];
    ret += (center_d - 1) * countr(&data, [0, 0], 196)[0];
    ret += (center_d - 1) * countr(&data, [0, 130], 196)[0];
    ret += (center_d - 1) * countr(&data, [130, 0], 196)[0];
    ret += (center_d - 1) * countr(&data, [130, 130], 196)[0];
    ret += (even_maps_in_grid - odd_maps_in_grid)
        * (southwest[1] + southeast[1] + center[1] + northwest[1] + northeast[1]);
    ret +=
        odd_maps_in_grid * (southwest[2] + southeast[2] + center[2] + northwest[2] + northeast[2]);
    Ok(ret as i64)
}

fn countr(inp: &[Vec<bool>], start: [usize; 2], len: usize) -> [usize; 3] {
    let mut leftpos = vec![vec![false; inp.len()]; inp.len()];
    let mut rightpos = vec![vec![false; inp.len()]; inp.len()];
    let mut events: VecDeque<[usize; 2]> = VecDeque::from([start]);
    let mut temp_holder = VecDeque::new();
    let mut ret = [0, 0, 0];
    for i in 0..(len + 5) {
        while let Some(evn) = events.pop_front() {
            if i % 2 == 0 {
                let old = rightpos[evn[0]][evn[1]];
                rightpos[evn[0]][evn[1]] = inp[evn[0]][evn[1]];
                if old != rightpos[evn[0]][evn[1]] {
                    (evn[0] + 1 < inp.len()).then(|| temp_holder.push_back([evn[0] + 1, evn[1]]));
                    evn[0]
                        .checked_sub(1)
                        .map(|x| temp_holder.push_back([x, evn[1]]));
                    (evn[1] + 1 < inp.len()).then(|| temp_holder.push_back([evn[0], evn[1] + 1]));
                    evn[1]
                        .checked_sub(1)
                        .map(|y| temp_holder.push_back([evn[0], y]));
                }
            } else {
                let old = leftpos[evn[0]][evn[1]];
                leftpos[evn[0]][evn[1]] = inp[evn[0]][evn[1]];
                if old != leftpos[evn[0]][evn[1]] {
                    if evn[0] + 1 < inp.len() {
                        temp_holder.push_back([evn[0] + 1, evn[1]])
                    }
                    if let Some(x) = evn[0].checked_sub(1) {
                        temp_holder.push_back([x, evn[1]])
                    }
                    if evn[1] + 1 < inp.len() {
                        temp_holder.push_back([evn[0], evn[1] + 1])
                    }
                    if let Some(y) = evn[1].checked_sub(1) {
                        temp_holder.push_back([evn[0], y])
                    }
                }
            }
        }
        events.append(&mut temp_holder);
        if i == len - 1 {
            ret = [
                leftpos
                    .iter()
                    .flat_map(|x| x.iter().filter(|&&x| x))
                    .count(),
                rightpos
                    .iter()
                    .flat_map(|x| x.iter().filter(|&&x| x))
                    .count(),
                0,
            ]
        } else if i == len + 4 {
            ret[2] = leftpos
                .iter()
                .enumerate()
                .flat_map(|(i, x)| {
                    x.iter().enumerate().filter(move |&(j, &t)| {
                        t && i.abs_diff(start[0]) + j.abs_diff(start[1]) < len
                    })
                })
                .count()
                + rightpos
                    .iter()
                    .enumerate()
                    .flat_map(|(i, x)| {
                        x.iter().enumerate().filter(move |&(j, &t)| {
                            t && i.abs_diff(start[0]) + j.abs_diff(start[1]) < len
                        })
                    })
                    .count()
        }
    }
    ret
}

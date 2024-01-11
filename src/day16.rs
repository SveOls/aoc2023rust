use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("day16")?;
    println!("day 16a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 16b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let data: Vec<Vec<_>> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    let beam = ([0, 0], 1);
    let mut que = VecDeque::new();
    que.push_front(beam);
    let mut visited = HashSet::new();
    while let Some(beam) = que.pop_front() {
        // dbg!(beam);
        if !visited.insert(beam) {
            continue;
        }
        match data[beam.0[0]][beam.0[1]] {
            '|' if beam.1 == 1 || beam.1 == 3 => {
                if beam.0[0] < data.len() - 1 {
                    que.push_front(([beam.0[0] + 1, beam.0[1]], 2));
                }
                if beam.0[0] > 0 {
                    que.push_front(([beam.0[0] - 1, beam.0[1]], 0));
                }
            }
            '-' if beam.1 == 0 || beam.1 == 2 => {
                if beam.0[1] < data[0].len() - 1 {
                    que.push_front(([beam.0[0], beam.0[1] + 1], 1));
                }
                if beam.0[1] > 0 {
                    que.push_front(([beam.0[0], beam.0[1] - 1], 3));
                }
            }
            '/' => match beam.1 {
                0 => {
                    if beam.0[1] < data[0].len() - 1 {
                        que.push_front(([beam.0[0], beam.0[1] + 1], 1));
                    }
                }
                1 => {
                    if beam.0[0] > 0 {
                        que.push_front(([beam.0[0] - 1, beam.0[1]], 0));
                    }
                }
                2 => {
                    if beam.0[1] > 0 {
                        que.push_front(([beam.0[0], beam.0[1] - 1], 3));
                    }
                }
                3 => {
                    if beam.0[0] < data.len() - 1 {
                        que.push_front(([beam.0[0] + 1, beam.0[1]], 2));
                    }
                }
                _ => panic!(),
            },
            '\\' => match beam.1 {
                0 => {
                    if beam.0[1] > 0 {
                        que.push_front(([beam.0[0], beam.0[1] - 1], 3));
                    }
                }
                1 => {
                    if beam.0[0] < data.len() - 1 {
                        que.push_front(([beam.0[0] + 1, beam.0[1]], 2));
                    }
                }
                2 => {
                    if beam.0[1] < data[0].len() - 1 {
                        que.push_front(([beam.0[0], beam.0[1] + 1], 1));
                    }
                }
                3 => {
                    if beam.0[0] > 0 {
                        que.push_front(([beam.0[0] - 1, beam.0[1]], 0));
                    }
                }
                _ => panic!(),
            },
            '.' | '|' | '-' => match beam.1 {
                0 => {
                    if beam.0[0] > 0 {
                        que.push_front(([beam.0[0] - 1, beam.0[1]], beam.1));
                    }
                }
                1 => {
                    if beam.0[1] < data[0].len() - 1 {
                        que.push_front(([beam.0[0], beam.0[1] + 1], beam.1));
                    }
                }
                2 => {
                    if beam.0[0] < data.len() - 1 {
                        que.push_front(([beam.0[0] + 1, beam.0[1]], beam.1));
                    }
                }
                3 => {
                    if beam.0[1] > 0 {
                        que.push_front(([beam.0[0], beam.0[1] - 1], beam.1));
                    }
                }
                _ => unreachable!(),
            },
            a => panic!("{a}"),
        }
    }
    let ret = visited
        .into_iter()
        .map(|x| x.0)
        .collect::<HashSet<_>>()
        .len();
    Ok(ret as i64)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let data: Vec<Vec<_>> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    let mut ret = 0;
    for (i, (j, d)) in (0..data.len())
        .zip(std::iter::repeat((0, 1)))
        .chain((0..data.len()).zip(std::iter::repeat((data.len() - 1, 3))))
        .chain((std::iter::repeat(0)).zip((0..data[0].len()).zip(std::iter::repeat(2))))
        .chain(
            (std::iter::repeat(data.len() - 1)).zip((0..data[0].len()).zip(std::iter::repeat(0))),
        )
    {
        // println!("{} {} {}", i, j, d);
        let beam = ([i, j], d);
        let mut que = VecDeque::new();
        que.push_front(beam);
        let mut visited = HashSet::new();
        while let Some(beam) = que.pop_front() {
            // dbg!(beam);
            if !visited.insert(beam) {
                continue;
            }
            match data[beam.0[0]][beam.0[1]] {
                '|' if beam.1 == 1 || beam.1 == 3 => {
                    if beam.0[0] < data.len() - 1 {
                        que.push_front(([beam.0[0] + 1, beam.0[1]], 2));
                    }
                    if beam.0[0] > 0 {
                        que.push_front(([beam.0[0] - 1, beam.0[1]], 0));
                    }
                }
                '-' if beam.1 == 0 || beam.1 == 2 => {
                    if beam.0[1] < data[0].len() - 1 {
                        que.push_front(([beam.0[0], beam.0[1] + 1], 1));
                    }
                    if beam.0[1] > 0 {
                        que.push_front(([beam.0[0], beam.0[1] - 1], 3));
                    }
                }
                '/' => match beam.1 {
                    0 => {
                        if beam.0[1] < data[0].len() - 1 {
                            que.push_front(([beam.0[0], beam.0[1] + 1], 1));
                        }
                    }
                    1 => {
                        if beam.0[0] > 0 {
                            que.push_front(([beam.0[0] - 1, beam.0[1]], 0));
                        }
                    }
                    2 => {
                        if beam.0[1] > 0 {
                            que.push_front(([beam.0[0], beam.0[1] - 1], 3));
                        }
                    }
                    3 => {
                        if beam.0[0] < data.len() - 1 {
                            que.push_front(([beam.0[0] + 1, beam.0[1]], 2));
                        }
                    }
                    _ => panic!(),
                },
                '\\' => match beam.1 {
                    0 => {
                        if beam.0[1] > 0 {
                            que.push_front(([beam.0[0], beam.0[1] - 1], 3));
                        }
                    }
                    1 => {
                        if beam.0[0] < data.len() - 1 {
                            que.push_front(([beam.0[0] + 1, beam.0[1]], 2));
                        }
                    }
                    2 => {
                        if beam.0[1] < data[0].len() - 1 {
                            que.push_front(([beam.0[0], beam.0[1] + 1], 1));
                        }
                    }
                    3 => {
                        if beam.0[0] > 0 {
                            que.push_front(([beam.0[0] - 1, beam.0[1]], 0));
                        }
                    }
                    _ => panic!(),
                },
                '.' | '|' | '-' => match beam.1 {
                    0 => {
                        if beam.0[0] > 0 {
                            que.push_front(([beam.0[0] - 1, beam.0[1]], beam.1));
                        }
                    }
                    1 => {
                        if beam.0[1] < data[0].len() - 1 {
                            que.push_front(([beam.0[0], beam.0[1] + 1], beam.1));
                        }
                    }
                    2 => {
                        if beam.0[0] < data.len() - 1 {
                            que.push_front(([beam.0[0] + 1, beam.0[1]], beam.1));
                        }
                    }
                    3 => {
                        if beam.0[1] > 0 {
                            que.push_front(([beam.0[0], beam.0[1] - 1], beam.1));
                        }
                    }
                    _ => unreachable!(),
                },
                a => panic!("{a}"),
            }
        }
        ret = ret.max(
            visited
                .into_iter()
                .map(|x| x.0)
                .collect::<HashSet<_>>()
                .len(),
        );
    }
    Ok(ret as i64)
}

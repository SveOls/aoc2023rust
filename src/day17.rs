use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    error::Error,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("adventofcode.com_2023_day_17_input.txt")?;
    println!("day 17a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 17b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

#[derive(PartialEq, Eq, Debug)]
struct State {
    in_row: u8,
    direction: u8,
    position: [usize; 2],
    heat: u32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat.cmp(&other.heat)
    }
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let data: Vec<Vec<_>> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|y| {
            y.chars()
                .map(|x| x.to_digit(10))
                .collect::<Option<Vec<_>>>()
        })
        .try_collect()
        .unwrap();

    let mut priority = BinaryHeap::new();

    priority.push(Reverse(State {
        in_row: 0,
        direction: 1,
        position: [0, 0],
        heat: 0,
    }));
    priority.push(Reverse(State {
        in_row: 0,
        direction: 2,
        position: [0, 0],
        heat: 0,
    }));

    let mut visited = HashSet::new();

    let mut res = None;

    while let Some(Reverse(state)) = priority.pop() {
        if state.position == [data.len() - 1, data[0].len() - 1] {
            res = Some(state.heat);
            break;
        }
        // println!("{:?}", state);
        if !visited.insert((state.position, state.direction, state.in_row)) {
            continue;
        } else {
            // for i in state.in_row..=max_row {
            //     visited.insert((state.position, state.direction, i));
            // }
        }
        if state.in_row != 0 {
            priority.push(Reverse(State {
                in_row: 0,
                direction: (state.direction + 5) % 4,
                position: state.position,
                heat: state.heat,
            }));
            priority.push(Reverse(State {
                in_row: 0,
                direction: (state.direction + 3) % 4,
                position: state.position,
                heat: state.heat,
            }));
        }
        if let Some((advance_pos, heat)) = match state.direction {
            0 => state.position[0]
                .checked_sub(1)
                .map(|x| ([x, state.position[1]], data[x][state.position[1]])),
            1 => data[state.position[0]]
                .get(state.position[1] + 1)
                .copied()
                .map(|x| ([state.position[0], state.position[1] + 1], x)),
            2 => data.get(state.position[0] + 1).map(|a| {
                (
                    [state.position[0] + 1, state.position[1]],
                    a[state.position[1]],
                )
            }),
            3 => state.position[1]
                .checked_sub(1)
                .map(|y| ([state.position[0], y], data[state.position[0]][y])),
            _ => panic!(),
        } && state.in_row < 3
        {
            priority.push(Reverse(State {
                in_row: state.in_row + 1,
                direction: state.direction,
                position: advance_pos,
                heat: state.heat + heat,
            }));
        }
    }
    Ok(res.unwrap() as i64)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let data: Vec<Vec<_>> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|y| {
            y.chars()
                .map(|x| x.to_digit(10))
                .collect::<Option<Vec<_>>>()
        })
        .try_collect()
        .unwrap();

    let mut priority = BinaryHeap::new();

    priority.push(Reverse(State {
        in_row: 0,
        direction: 1,
        position: [0, 0],
        heat: 0,
    }));
    priority.push(Reverse(State {
        in_row: 0,
        direction: 2,
        position: [0, 0],
        heat: 0,
    }));
    let mut visited = HashSet::new();

    let mut res = None;

    while let Some(Reverse(state)) = priority.pop() {
        if state.position == [data.len() - 1, data[0].len() - 1] && state.in_row >= 4 {
            res = Some(state.heat);
            break;
        }
        // println!("{:?}", state);
        if !visited.insert((state.position, state.direction, state.in_row)) {
            continue;
        } else {
            // for i in state.in_row..=max_row {
            //     visited.insert((state.position, state.direction, i));
            // }
        }
        if state.in_row != 0 && state.in_row >= 4 {
            priority.push(Reverse(State {
                in_row: 0,
                direction: (state.direction + 5) % 4,
                position: state.position,
                heat: state.heat,
            }));
            priority.push(Reverse(State {
                in_row: 0,
                direction: (state.direction + 3) % 4,
                position: state.position,
                heat: state.heat,
            }));
        }
        if let Some((advance_pos, heat)) = match state.direction {
            0 => state.position[0]
                .checked_sub(1)
                .map(|x| ([x, state.position[1]], data[x][state.position[1]])),
            1 => data[state.position[0]]
                .get(state.position[1] + 1)
                .copied()
                .map(|x| ([state.position[0], state.position[1] + 1], x)),
            2 => data.get(state.position[0] + 1).map(|a| {
                (
                    [state.position[0] + 1, state.position[1]],
                    a[state.position[1]],
                )
            }),
            3 => state.position[1]
                .checked_sub(1)
                .map(|y| ([state.position[0], y], data[state.position[0]][y])),
            _ => panic!(),
        } && state.in_row < 10
        {
            priority.push(Reverse(State {
                in_row: state.in_row + 1,
                direction: state.direction,
                position: advance_pos,
                heat: state.heat + heat,
            }));
        }
    }
    Ok(res.unwrap() as i64)
}

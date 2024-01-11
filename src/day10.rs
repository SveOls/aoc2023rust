use std::{
    collections::HashSet,
    error::Error,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("adventofcode.com_2023_day_10_input.txt")?;
    println!("day 10a: {}", parta(&file)?);
    println!("day 10b: {}", partb(&file)?);
    Ok(())
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    Northeast,
    Northwest,
    Southwest,
    Southeast,
    None,
    Animal,
}

impl Pipe {
    fn from_direction(self, inp: u8) -> bool {
        use Pipe::*;
        match inp {
            0 => self == Vertical || self == Northwest || self == Northeast,
            1 => self == Horizontal || self == Southeast || self == Northeast,
            2 => self == Vertical || self == Southeast || self == Southwest,
            3 => self == Horizontal || self == Northwest || self == Southwest,
            _ => panic!(),
        }
    }
    fn adjacents(self, [a, b]: [usize; 2]) -> [[usize; 2]; 2] {
        match self {
            Pipe::Vertical => [[a + 1, b], [a - 1, b]],
            Pipe::Horizontal => [[a, b + 1], [a, b - 1]],
            Pipe::Northeast => [[a - 1, b], [a, b + 1]],
            Pipe::Northwest => [[a - 1, b], [a, b - 1]],
            Pipe::Southwest => [[a + 1, b], [a, b - 1]],
            Pipe::Southeast => [[a + 1, b], [a, b + 1]],
            _ => panic!(),
        }
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        use Pipe::*;
        match value {
            '.' => None,
            '|' => Vertical,
            '-' => Horizontal,
            'F' => Southeast,
            'J' => Northwest,
            '7' => Southwest,
            'L' => Northeast,
            'S' => Animal,
            a => unreachable!("{a}"),
        }
    }
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let maze: Vec<Vec<Pipe>> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|x| x.into()).collect())
        .collect();
    let mut res = 1;
    let start = maze
        .iter()
        .enumerate()
        .flat_map(|x| x.1.iter().enumerate().map(move |y| (x.0, y.0, y.1)))
        .filter(|x| *x.2 == Pipe::Animal)
        .map(|x| [x.0, x.1])
        .next()
        .unwrap();
    let home: Pipe = match [
        maze[start[0]][start[1] - 1].from_direction(1),
        maze[start[0]][start[1] + 1].from_direction(3),
        maze[start[0] - 1][start[1]].from_direction(2),
        maze[start[0] + 1][start[1]].from_direction(0),
    ] {
        [true, true, false, false] => '-'.into(),
        [true, false, true, false] => 'J'.into(),
        [true, false, false, true] => '7'.into(),
        [false, true, true, false] => 'L'.into(),
        [false, true, false, true] => 'F'.into(),
        [false, false, true, true] => '|'.into(),
        a => unimplemented!("{a:?}"),
    };
    let [mut left, mut right] = home.adjacents(start);
    let mut visited = HashSet::new();
    visited.insert(start);
    visited.insert(left);
    visited.insert(right);
    loop {
        let lefts: Vec<_> = maze[left[0]][left[1]]
            .adjacents(left)
            .into_iter()
            .filter(|x| visited.insert(*x))
            .collect();
        if lefts.is_empty() {
            break;
        } else if lefts.len() > 1 {
            panic!("{:?}", lefts);
        }
        left = lefts[0];

        res += 1;

        let rights: Vec<_> = maze[right[0]][right[1]]
            .adjacents(right)
            .into_iter()
            .filter(|x| visited.insert(*x))
            .collect();
        if rights.is_empty() {
            break;
        } else if rights.len() > 1 {
            panic!()
        }
        right = rights[0];
    }
    Ok(res as i64)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let maze: Vec<Vec<Pipe>> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|x| x.into()).collect())
        .collect();
    let start = maze
        .iter()
        .enumerate()
        .flat_map(|x| x.1.iter().enumerate().map(move |y| (x.0, y.0, y.1)))
        .filter(|x| *x.2 == Pipe::Animal)
        .map(|x| [x.0, x.1])
        .next()
        .unwrap();
    let home: Pipe = match [
        maze[start[0]][start[1] - 1].from_direction(1),
        maze[start[0]][start[1] + 1].from_direction(3),
        maze[start[0] - 1][start[1]].from_direction(2),
        maze[start[0] + 1][start[1]].from_direction(0),
    ] {
        [true, true, false, false] => '-'.into(),
        [true, false, true, false] => 'J'.into(),
        [true, false, false, true] => '7'.into(),
        [false, true, true, false] => 'L'.into(),
        [false, true, false, true] => 'F'.into(),
        [false, false, true, true] => '|'.into(),
        a => unimplemented!("{a:?}"),
    };
    let [mut left, mut right] = home.adjacents(start);
    let mut visited = HashSet::new();
    visited.insert(start);
    visited.insert(left);
    visited.insert(right);
    loop {
        let lefts: Vec<_> = maze[left[0]][left[1]]
            .adjacents(left)
            .into_iter()
            .filter(|x| visited.insert(*x))
            .collect();
        if lefts.is_empty() {
            break;
        } else if lefts.len() > 1 {
            panic!("{:?}", lefts);
        }
        left = lefts[0];

        let rights: Vec<_> = maze[right[0]][right[1]]
            .adjacents(right)
            .into_iter()
            .filter(|x| visited.insert(*x))
            .collect();
        if rights.is_empty() {
            break;
        } else if rights.len() > 1 {
            panic!()
        }
        right = rights[0];
    }
    let mut counter = 0;
    let mut count;
    let mut last_bend;
    let mut maze = maze;
    maze[start[0]][start[1]] = home;
    let maze = maze;
    for i in 0..maze.len() {
        count = false;
        last_bend = None;
        for j in 0..maze[i].len() {
            if visited.contains(&[i, j]) {
                match maze[i][j] {
                    Pipe::Vertical => count = !count,
                    Pipe::Horizontal => {}
                    Pipe::Northeast => last_bend = Some(Pipe::Northeast),
                    Pipe::Northwest => {
                        if last_bend == Some(Pipe::Southeast) {
                            count = !count
                        }
                        last_bend = None;
                    }
                    Pipe::Southwest => {
                        if last_bend == Some(Pipe::Northeast) {
                            count = !count
                        }
                        last_bend = None;
                    }
                    Pipe::Southeast => last_bend = Some(Pipe::Southeast),
                    Pipe::None => unreachable!(),
                    Pipe::Animal => unreachable!(),
                }
            } else if count {
                counter += 1;
            }
        }
    }
    Ok(counter as i64)
}

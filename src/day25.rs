use std::{
    array::IntoIter,
    collections::HashMap,
    error::Error,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("adventofcode.com_2023_day_25_input.txt")?;
    println!("day 25a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 25b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Connection {
    left: [char; 3],
    right: [char; 3],
}

impl Connection {
    fn new(left: [char; 3], right: [char; 3]) -> Self {
        Self {
            left: left.min(right),
            right: right.max(left),
        }
    }
    fn get(&self, rhs: &[char; 3]) -> Option<&[char; 3]> {
        if &self.left == rhs {
            Some(&self.right)
        } else if &self.right == rhs {
            Some(&self.left)
        } else {
            None
        }
    }
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut connections: HashMap<Connection, usize> = HashMap::new();
    let mut all = HashMap::new();
    let mut dataer = HashMap::new();
    for line in inp.split('\n').filter(|x| !x.is_empty()) {
        let mut letter = line
            .chars()
            .filter(|x| x.is_ascii_alphabetic())
            .array_chunks();
        let left: [char; 3] = letter.next().unwrap();
        all.insert(left, 1);
        for right in letter.by_ref() {
            all.insert(right, 1);
            connections.insert(Connection::new(right, left), 1);
            dataer.insert(Connection::new(right, left), 1);
        }
        assert!(
            letter
                .into_remainder()
                .map(IntoIter::<char, 3>::count)
                .unwrap_or(0)
                == 0
        );
    }
    let mut edges = i64::MAX;
    let mut leftind = None;
    let mut rightind = None;
    while edges > 3 {
        let mut connections_inner = connections.clone();
        all.values_mut().for_each(|x| *x = 1);
        while connections_inner.len() > 1 {
            let index = rand::random::<usize>() % connections_inner.len();
            let test_index = connections_inner
                .iter()
                .max_by(|x, y| x.1.cmp(y.1))
                .unwrap();
            let a = if *test_index.1 > 3 {
                test_index.0.clone()
            } else {
                connections_inner.keys().nth(index).unwrap().clone()
            };
            let b = connections_inner.remove_entry(&a).unwrap();
            let left = b.0.left;
            let stuffs = connections_inner
                .extract_if(|x, _| x.get(&left).is_some())
                .collect::<Vec<_>>();
            let right = b.0.right;
            // println!("merge {:?}\n", a);
            // println!("{:?}", connections_inner);
            let l = *all.get(&left).unwrap();
            let r = *all.get(&right).unwrap();
            all.get_mut(&left).map(|x| *x += r).unwrap();
            all.get_mut(&right).map(|x| *x += l).unwrap();
            for j in stuffs {
                // println!("delete {:?}\n", j);
                // println!("replace {:?}\n", Connection::new(j.0.get(&left).copied().unwrap(), right));

                *connections_inner
                    .entry(Connection::new(j.0.get(&left).copied().unwrap(), right))
                    .or_default() += j.1;
            }
            // println!("{:?}\n", connections_inner);
            // panic!();
        }
        // println!("{:?}", connections_inner);
        edges = connections_inner.values().sum::<usize>() as i64;
        // println!("{}", edges);
        // dbg!(&connections_inner);
        let a = connections_inner.iter().next().unwrap();
        // println!("{:?}, {:?}", all.get(&a.0.left), all.get(&a.0.right));
        leftind = Some(a.0.left);
        rightind = Some(a.0.right);
    }
    println!(
        "{:?}, {:?}",
        all.get(&leftind.unwrap()),
        all.get(&rightind.unwrap())
    );
    Ok(*all.get(&leftind.unwrap()).unwrap() as i64 * *all.get(&rightind.unwrap()).unwrap() as i64)

    // for (&x, &y) in all.iter().flat_map(|x| std::iter::repeat(x).zip(&all)) {
    //     connections.entry(Connection::new(x, y)).or_default();
    // }
    // let mut groups: Vec<HashSet<[char; 3]>> = Vec::new();

    // connections.remove(&Connection::new(['h', 'f', 'x'], ['p', 'z', 'l']));
    // connections.remove(&Connection::new(['b', 'v', 'b'], ['c', 'm', 'g']));
    // connections.remove(&Connection::new(['n', 'v', 'd'], ['j', 'q', 't']));

    // let keys = connections.keys().cloned().collect::<Vec<_>>();
    // for i in keys {
    //     recurs(i, &mut connections);
    // }
    // for i in connections {
    //     println!("{:?}", i);
    // }

    // for i in recurs(&['x', 'h', 'k'], &mut connections, HashSet::new()) {
    //     println!("{:?}", i);
    // }
    // let old_c = connections.clone();
    // for (con, &mut val) in &mut connections {
    //     let left = con.left;
    //     let right = con.right;
    //     let lefts = old_c.iter().filter_map(|x| x.0.get(&left)).copied().collect::<Vec<_>>();
    //     let rights = old_c.iter().filter_map(|x| x.0.get(&right)).copied().collect::<Vec<_>>();
    //     // *val =
    //     // dbg!(left);
    //     // dbg!(lefts);
    //     // dbg!(right);
    //     // dbg!(rights);
    //     // panic!();
    // }
}

// fn recurs(point: Connection, info: &mut HashMap<Connection, usize>) {
//     let start = point.left;
//     let end = point.right;
//     let starter = info
//         .keys()
//         .filter_map(|x| x.get(&start))
//         .copied()
//         .collect::<HashSet<_>>();
//     let ender = info
//         .keys()
//         .filter_map(|x| x.get(&end))
//         .copied()
//         .collect::<HashSet<_>>();
//     *info.entry(point).or_default() += starter.intersection(&ender).count();
// }

fn partb(_inp: &str) -> Result<i64, Box<dyn Error>> {
    Ok(0)
}

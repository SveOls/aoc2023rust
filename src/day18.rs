use std::
    error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("adventofcode.com_2023_day_18_input.txt")?;
    println!("day 18a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 18b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut pos: [i64; 2] = [0, 0];
    let mut data = vec![pos];
    for line in inp.split('\n').filter(|x| !x.is_empty()) {
        let mut thing = line.split_whitespace();
        let direction: char = thing.next().unwrap().chars().next().unwrap();
        let length: i64 = thing.next().unwrap().parse()?;
        match direction {
            'U' => pos = [pos[0] - length, pos[1]],
            'R' => pos = [pos[0], pos[1] + length],
            'D' => pos = [pos[0] + length, pos[1]],
            'L' => pos = [pos[0], pos[1] - length],
            _ => panic!(),
        }
        data.push(pos);
    }
    data.pop();
    // dbg!(&data);
    // panic!("{}", data.windows(2).map(|x| (x[0][0] + x[0][1]).abs_diff(x[1][0] + x[1][1])).inspect(|x| println!("{x}")).sum::<u64>());
    // for &[back, here, front] in data.array_windows() {}
    let mut res = 0;
    // println!(
    //     "{} {}",
    //     data.iter().map(|x| x[0]).min().unwrap(),
    //     data.iter().map(|x| x[0]).max().unwrap()
    // );
    for i in data.iter().map(|x| x[0]).min().unwrap()..=data.iter().map(|x| x[0]).max().unwrap() {
        // if i > 1 {
        //     break;
        // }
        let mut count = vec![0];
        let mut inner = false;
        let mut between_corner = false;
        let mut prev = false;
        for j in data.iter().map(|x| x[1]).min().unwrap()..=data.iter().map(|x| x[1]).max().unwrap()
        {
            // dbg!(inner);
            // dbg!(between_corner);
            if let Some(pos) = data.iter().position(|&x| x == [i, j]) {
                assert!(
                    data[(pos + data.len() - 1) % data.len()][0] != data[(pos + 1) % data.len()][0]
                );
                if i == 2 {
                    // dbg!(between_corner);
                    // dbg!(pos);
                    // dbg!(data[pos]);
                    // dbg!(data[pos - 2]);
                    // dbg!(data[pos - 1]);
                    // dbg!(data[(pos + 2) % data.len()]);
                    // dbg!(data[(pos + 1) % data.len()]);
                    // dbg!(inner);
                }
                if !between_corner {
                    count.push(0);
                }
                if data[pos][0] == data[(pos + 1) % data.len()][0] {
                    if !between_corner
                        && ((data[(pos + data.len() - 1) % data.len()][0] < i)
                            ^ (data[(pos + 2) % data.len()][0] < i))
                    {
                        inner = !inner;
                    }
                } else if data[pos][0] == data[(pos + data.len() - 1) % data.len()][0] {
                    if !between_corner
                        && ((data[(pos + data.len() - 2) % data.len()][0] < i)
                            ^ (data[(pos + 1) % data.len()][0] < i))
                    {
                        inner = !inner;
                    }
                }
                between_corner = !between_corner;
                prev = true;
                if let Some(x) = count.last_mut() {
                    *x += 1;
                }
                if i == 2 {
                    // dbg!(inner);
                }
            } else if data
                .array_windows::<2>()
                .chain(std::iter::once(&[data[data.len() - 1], data[0]]))
                .any(|[x, y]| {
                    i == i.clamp(x[0].min(y[0]), x[0].max(y[0]))
                        && j == j.clamp(x[1].min(y[1]), y[1].max(x[1]))
                })
            {
                if !between_corner {
                    inner = !inner;
                    count.push(0);
                }
                prev = true;
                if let Some(x) = count.last_mut() {
                    *x += 1
                }
            } else {
                if prev {
                    count.push(0);
                    prev = false;
                }
                if let Some(x) = count.last_mut() && inner {
                    *x += 1
                }
            }
        }
        // println!("{}", i);
        // println!("{:?}", count);
        // println!("{:?}", count.);
        res += count.iter().sum::<i64>();
    }
    // unimplemented!()

    // 0 1 4 1 1 1 1 1 0
    // |_| yes
    // ||_| yes
    // || no
    // ||_|| maybe??
    Ok(res)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut pos: [i64; 2] = [0, 0];
    let mut data = vec![pos];
    for line in inp.split('\n').filter(|x| !x.is_empty()) {
        let thing: Vec<_> = line
            .split_whitespace()
            .nth(2)
            .unwrap()
            .chars()
            .filter(|x| x.is_ascii_hexdigit())
            .collect();
        let length: i64 = i64::from_str_radix(&thing[..5].iter().collect::<String>(), 16)?;
        match thing.last().unwrap() {
            '3' => pos = [pos[0] - length, pos[1]],
            '0' => pos = [pos[0], pos[1] + length],
            '1' => pos = [pos[0] + length, pos[1]],
            '2' => pos = [pos[0], pos[1] - length],
            _ => panic!(),
        }
        data.push(pos);
    }
    data.pop();
    // dbg!(&data);
    // panic!("{}", data.windows(2).map(|x| (x[0][0] + x[0][1]).abs_diff(x[1][0] + x[1][1])).inspect(|x| println!("{x}")).sum::<u64>());
    // for &[back, here, front] in data.array_windows() {}
    let mut res = 0;
    // println!(
    //     "{} {}",
    //     data.iter().map(|x| x[0]).min().unwrap(),
    //     data.iter().map(|x| x[0]).max().unwrap()
    // );
    // let min = data.iter().map(|x| x[0]).min().unwrap();
    // let max = data.iter().map(|x| x[0]).max().unwrap();
    let mut ivals: Vec<_> = data.iter().map(|x| x[0]).collect();
    let mut jvals: Vec<_> = data.iter().map(|x| x[1]).collect();
    ivals.sort();
    ivals.dedup();
    jvals.sort();
    jvals.dedup();
    let separations: Vec<_> = ivals
        .clone()
        .windows(2)
        .map(|x| (x[0] + 1, x[1] - x[0] - 1))
        .filter(|x| x.1 != 0)
        .collect();
    let mut ep = separations.into_iter();
    for (i, n) in ivals
        .iter()
        .copied()
        .map(|x| (x, 1))
        .intersperse_with(|| ep.next().unwrap())
    {
        // println!("{}", (i - min) as f64 / (max - min) as f64);
        // if i > 1 {
        //     break;
        // }
        let mut count = vec![0];
        let mut inner = false;
        let mut between_corner = false;
        let mut prev = false;
        let separations: Vec<_> = jvals
            .clone()
            .windows(2)
            .map(|x| (x[0] + 1, x[1] - x[0] - 1))
            .filter(|x| x.1 != 0)
            .collect();
        let mut ep = separations.into_iter();
        for (j, m) in jvals
            .iter()
            .copied()
            .map(|x| (x, 1))
            .intersperse_with(|| ep.next().unwrap())
        {
            // dbg!(inner);
            // dbg!(between_corner);
            if let Some(pos) = data.iter().position(|&x| x == [i, j]) {
                if !between_corner {
                    count.push(0);
                }
                if data[pos][0] == data[(pos + 1) % data.len()][0] {
                    if !between_corner
                        && ((data[(pos + data.len() - 1) % data.len()][0] < i)
                            ^ (data[(pos + 2) % data.len()][0] < i))
                    {
                        inner = !inner;
                    }
                } else if data[pos][0] == data[(pos + data.len() - 1) % data.len()][0] {
                    if !between_corner
                        && ((data[(pos + data.len() - 2) % data.len()][0] < i)
                            ^ (data[(pos + 1) % data.len()][0] < i))
                    {
                        inner = !inner;
                    }
                }
                between_corner = !between_corner;
                prev = true;
                if let Some(x) = count.last_mut() {
                    *x += m;
                }
                if i == 2 {
                    // dbg!(inner);
                }
            } else if data
                .array_windows::<2>()
                .chain(std::iter::once(&[data[data.len() - 1], data[0]]))
                .any(|[x, y]| {
                    i == i.clamp(x[0].min(y[0]), x[0].max(y[0]))
                        && j == j.clamp(x[1].min(y[1]), y[1].max(x[1]))
                })
            {
                if !between_corner {
                    inner = !inner;
                    count.push(0);
                }
                prev = true;
                if let Some(x) = count.last_mut() {
                    *x += m;
                }
            } else {
                if prev {
                    count.push(0);
                    prev = false;
                }
                if let Some(x) = count.last_mut() && inner {
                    *x += m;
                }
            }
        }
        // println!("{}", i);
        // println!("{:?}", count);
        // println!("{:?}", count.);
        res += n * count.iter().sum::<i64>();
    }
    // unimplemented!()

    // 0 1 4 1 1 1 1 1 0
    // |_| yes
    // ||_| yes
    // || no
    // ||_|| maybe??
    Ok(res)
}

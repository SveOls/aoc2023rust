use std::
    error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("day9")?;
    println!("day 9a: {}", parta(&file)?);
    println!("day 9b: {}", partb(&file)?);
    Ok(())
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut input = inp.split('\n').filter(|x| !x.is_empty());
    let mut res = 0;
    while let Some(data) = input
        .next()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i64>())
                .try_collect::<Vec<_>>()
        })
        .transpose()?
    {
        let mut data = vec![data];
        while data.iter().last().unwrap().iter().any(|&x| x != 0) {
            let mut new = Vec::new();
            for &[a, b] in data.iter().last().unwrap().array_windows() {
                new.push(b - a);
            }
            // println!("{new:?}");
            data.push(new);
        }
        for i in (0..data.len()).rev() {
            let next =
                data[i].last().unwrap() + data.get(i + 1).map(|x| x.last().unwrap()).unwrap_or(&0);
            data[i].push(next);
            // println!("{:?}", data[i]);
        }
        res += data[0].last().unwrap();
    }

    Ok(res)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut input = inp.split('\n').filter(|x| !x.is_empty());
    let mut res = 0;
    while let Some(data) = input
        .next()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i64>())
                .rev()
                .try_collect::<Vec<_>>()
        })
        .transpose()?
    {
        let mut data = vec![data];
        while data.iter().last().unwrap().iter().any(|&x| x != 0) {
            let mut new = Vec::new();
            for &[a, b] in data.iter().last().unwrap().array_windows() {
                new.push(b - a);
            }
            // println!("{new:?}");
            data.push(new);
        }
        for i in (0..data.len()).rev() {
            let next =
                data[i].last().unwrap() + data.get(i + 1).map(|x| x.last().unwrap()).unwrap_or(&0);
            data[i].push(next);
            // println!("{:?}", data[i]);
        }
        res += data[0].last().unwrap();
    }

    Ok(res)
}

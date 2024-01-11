use std::
    error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("day11")?;
    println!("day 11a: {}", parta(&file)?);
    println!("day 11b: {}", partb(&file)?);
    Ok(())
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let stars: Vec<[usize; 2]> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .enumerate()
        .flat_map(|y| y.1.chars().enumerate().map(move |l| (y.0, l.0, l.1)))
        // .inspect(|x| println!("{:?}", x))
        .filter(|x| x.2 == '#')
        .map(|x| [x.0, x.1])
        .collect();
    let mut res = 0;
    for i in 0..stars.iter().map(|x| x[0]).max().unwrap() {
        res += stars.iter().filter(|&&x| x[0] <= i).count()
            * stars.iter().filter(|&&x| x[0] > i).count()
            * stars.iter().any(|x| x[0] == i).then_some(1).unwrap_or(2);
    }
    for i in 0..stars.iter().map(|x| x[1]).max().unwrap() {
        res += stars.iter().filter(|&&x| x[1] <= i).count()
            * stars.iter().filter(|&&x| x[1] > i).count()
            * stars.iter().any(|x| x[1] == i).then_some(1).unwrap_or(2);
    }
    Ok(res as i64)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let now = std::time::Instant::now();
    let stars: Vec<[usize; 2]> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .enumerate()
        .flat_map(|y| y.1.chars().enumerate().map(move |l| (y.0, l.0, l.1)))
        .filter(|x| x.2 == '#')
        .map(|x| [x.0, x.1])
        .collect();
    let mut res = 0;
    for i in 0..stars.iter().map(|x| x[0]).max().unwrap() {
        res += stars.iter().filter(|&&x| x[0] <= i).count()
            * stars.iter().filter(|&&x| x[0] > i).count()
            * stars
                .iter()
                .any(|x| x[0] == i)
                .then_some(1)
                .unwrap_or(1000000);
    }
    for i in 0..stars.iter().map(|x| x[1]).max().unwrap() {
        res += stars.iter().filter(|&&x| x[1] <= i).count()
            * stars.iter().filter(|&&x| x[1] > i).count()
            * stars
                .iter()
                .any(|x| x[1] == i)
                .then_some(1)
                .unwrap_or(1000000);
    }
    println!("{:?}", now.elapsed());
    Ok(res as i64)
}

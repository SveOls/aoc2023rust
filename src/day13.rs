use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("day13")?;
    println!("day 13a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 13b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let a = inp.split('\n');
    let mut data = Vec::new();
    let mut temp = Vec::new();
    for b in a {
        if b.is_empty() {
            data.push(std::mem::take(&mut temp));
        } else {
            temp.push(b.chars().map(|c| c == '#').collect::<Vec<_>>());
        }
    }
    let mut res = 0;
    'outer: for table in data {
        for i in 1..table.len() {
            let mut check = true;
            for j in (2 * i).saturating_sub(table.len())..i {
                // println!("{} {} {}", i, j, 2 * i-j-1);
                if table[j] != table[2 * i - j - 1] {
                    check = false;
                }
            }
            if check {
                res += 100 * i;
                continue 'outer;
            }
        }
        for i in 1..table[0].len() {
            let mut check = true;
            for j in (2 * i).saturating_sub(table[0].len())..i {
                // println!("{} {} {}", i, j, 2 * i-j-1);
                if table.iter().any(|x| x[j] != x[2 * i - j - 1]) {
                    check = false;
                }
            }
            if check {
                res += i;
                continue 'outer;
            }
        }
    }
    Ok(res as i64)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let a = inp.split('\n');
    let mut data = Vec::new();
    let mut temp = Vec::new();
    for b in a {
        if b.is_empty() {
            data.push(std::mem::take(&mut temp));
        } else {
            temp.push(b.chars().map(|c| c == '#').collect::<Vec<_>>());
        }
    }
    let mut res = 0;
    'outer: for table in data {
        for i in 1..table.len() {
            let mut smudge = 0;
            for j in (2 * i).saturating_sub(table.len())..i {
                smudge += table[j]
                    .iter()
                    .zip(&table[2 * i - j - 1])
                    .filter(|x| x.0 != x.1)
                    .count();
            }
            if smudge == 1 {
                res += 100 * i;
                continue 'outer;
            }
        }
        for i in 1..table[0].len() {
            let mut smudge = 0;
            for j in (2 * i).saturating_sub(table[0].len())..i {
                smudge += table.iter().filter(|x| x[j] != x[2 * i - j - 1]).count();
            }
            if smudge == 1 {
                res += i;
                continue 'outer;
            }
        }
    }
    Ok(res as i64)
}

// 0==1
// 0==3, 1==2
// 0-1 3-2

// 1 | 1
// 2 | 3 2
// 3 | 5 4 3

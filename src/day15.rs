use std::
    error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("day15")?;
    println!("day 15a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 15b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let data = inp
        .split(',')
        .map(|x| x.trim())
        .map(|x| x.chars().fold(0, |acc, x| acc * 17 + x as u8 * 17))
        .map(|x| x as i64)
        .sum();
    Ok(data)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let data: Vec<_> = inp
        .split(',')
        .map(|x| x.trim())
        .map(|x| {
            (
                x.chars()
                    .take(x.chars().position(|x| !x.is_alphabetic()).unwrap())
                    .fold(0, |acc, x| acc * 17 + x as u8 * 17),
                x.chars()
                    .take(x.chars().position(|x| !x.is_alphabetic()).unwrap())
                    .collect::<String>(),
                x.chars()
                    .nth(x.chars().position(|x| !x.is_alphabetic()).unwrap())
                    .unwrap(),
                x.chars()
                    .skip(x.chars().position(|x| !x.is_alphabetic()).unwrap() + 1)
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap_or(0),
            )
        })
        .collect();
    let mut stuff = vec![Vec::<(String, i64)>::new(); 256];
    for (code, strin, operation, value) in data {
        let mut stuffer = std::mem::take(&mut stuff[code as usize]);
        match operation {
            '-' => {
                stuff[code as usize] = stuffer.into_iter().filter(|x| x.0 != strin).collect();
            }
            '=' => {
                if let Some(a) = stuffer.iter().position(|x| x.0 == strin) {
                    stuffer[a].1 = value;
                } else {
                    stuffer.push((strin, value));
                }
                stuff[code as usize] = stuffer.into_iter().collect();
            }
            _ => panic!(),
        }
        // stuff[code] = *
    }
    let res: usize = stuff
        .into_iter()
        .enumerate()
        .flat_map(|y| y.1.into_iter().enumerate().map(move |t| (y.0, t.0, t.1 .1)))
        .map(|x| (x.0 + 1) * (x.1 + 1) * x.2 as usize)
        .sum();
    Ok(res as i64)
}

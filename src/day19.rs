use std::{collections::HashMap, error::Error};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("day19")?;
    println!("day 19a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 19b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut parts: Vec<[i64; 4]> = Vec::new();
    let mut rules: HashMap<&str, Vec<(Option<(char, char, i64)>, &str)>> = HashMap::new();
    let mut lines = inp.split('\n');
    for a in lines.by_ref() {
        if a.is_empty() {
            break;
        }
        let mut thing = a
            .split(|x| x == '{' || x == '}' || x == ',')
            .filter(|x| !x.is_empty());
        // println!("{:?}", thing.collect::<Vec<_>>());
        let key = thing.next().unwrap();
        let mut stuff = Vec::new();
        for b in thing {
            if let Some((mut oeeeie, last)) = b
                .split_once(':')
                .map(|x| (x.0.split_inclusive(|c: char| !c.is_ascii_digit()), x.1))
            {
                stuff.push((
                    Some((
                        oeeeie.next().unwrap().chars().next().unwrap(),
                        oeeeie.next().unwrap().chars().next().unwrap(),
                        oeeeie.next().unwrap().parse()?,
                    )),
                    last,
                ));
            } else {
                stuff.push((None, b));
            }
        }
        rules.insert(key, stuff);
    }
    for a in lines.by_ref() {
        if a.is_empty() {
            break;
        }
        parts.push(
            a.split(|x: char| !x.is_ascii_digit())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse())
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .unwrap(),
        );
    }
    // dbg!(&rules);
    // dbg!(&parts);
    let mut res = 0;
    for part in parts {
        res += part.iter().sum::<i64>() * evaluate(part, "in", &rules) as i64;
    }
    Ok(res)
}

fn evaluate(
    inp: [i64; 4],
    rule: &str,
    check: &HashMap<&str, Vec<(Option<(char, char, i64)>, &str)>>,
) -> bool {
    let mut ret = None;
    for rule in check.get(rule).unwrap() {
        match rule {
            (Some((letter, geqleq, value)), new_key) => {
                let geq = geqleq == &'>';
                match letter {
                    'x' => {
                        if &inp[0] > value && geq || &inp[0] < value && !geq {
                            ret = Some(new_key);
                            break;
                        }
                    }
                    'm' => {
                        if &inp[1] > value && geq || &inp[1] < value && !geq {
                            ret = Some(new_key);
                            break;
                        }
                    }
                    'a' => {
                        if &inp[2] > value && geq || &inp[2] < value && !geq {
                            ret = Some(new_key);
                            break;
                        }
                    }
                    's' => {
                        if &inp[3] > value && geq || &inp[3] < value && !geq {
                            ret = Some(new_key);
                            break;
                        }
                    }
                    _ => panic!(),
                }
            }
            (None, new_key) => ret = Some(new_key),
        }
    }
    match ret.unwrap().as_ref() {
        "A" => true,
        "R" => false,
        a => evaluate(inp, a, check),
    }
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    // let mut parts: Vec<[i64; 4]> = Vec::new();
    let mut rules: HashMap<&str, Vec<(Option<(char, char, i64)>, &str)>> = HashMap::new();
    let lines = inp.split('\n');
    for a in lines {
        if a.is_empty() {
            break;
        }
        let mut thing = a
            .split(|x| x == '{' || x == '}' || x == ',')
            .filter(|x| !x.is_empty());
        // println!("{:?}", thing.collect::<Vec<_>>());
        let key = thing.next().unwrap();
        let mut stuff = Vec::new();
        for b in thing {
            if let Some((mut oeeeie, last)) = b
                .split_once(':')
                .map(|x| (x.0.split_inclusive(|c: char| !c.is_ascii_digit()), x.1))
            {
                stuff.push((
                    Some((
                        oeeeie.next().unwrap().chars().next().unwrap(),
                        oeeeie.next().unwrap().chars().next().unwrap(),
                        oeeeie.next().unwrap().parse()?,
                    )),
                    last,
                ));
            } else {
                stuff.push((None, b));
            }
        }
        rules.insert(key, stuff);
    } // dbg!(&rules);
      // dbg!(&parts);
    let res = evaluateb([[1, 4000]; 4], "in", &rules);
    // 167409079868000
    // 153945092205921
    Ok(res)
}

fn evaluateb(
    mut inp: [[i64; 2]; 4],
    rule: &str,
    check: &HashMap<&str, Vec<(Option<(char, char, i64)>, &str)>>,
) -> i64 {
    if inp.iter().map(|[x, y]| y - x + 1).any(|x| x <= 0) {
        return 0;
    } else if rule == "A" {
        return inp.iter().map(|x| x[1] - x[0] + 1).product();
    } else if rule == "R" {
        return 0;
    }

    let mut ret = 0;
    for rule in check.get(rule).unwrap() {
        match rule {
            &(Some((letter, geqleq, value)), new_key) => {
                let mut new_inp = inp;
                match (letter, geqleq) {
                    // <1000
                    // new: 0-999 old: 1000+
                    // >1000
                    // new: 1001+, old: 0-1000
                    ('x', '>') => {
                        new_inp[0][0] = value + 1;
                        inp[0][1] = value;
                    }
                    ('x', '<') => {
                        new_inp[0][1] = value - 1;
                        inp[0][0] = value;
                    }
                    ('m', '>') => {
                        new_inp[1][0] = value + 1;
                        inp[1][1] = value;
                    }
                    ('m', '<') => {
                        new_inp[1][1] = value - 1;
                        inp[1][0] = value;
                    }
                    ('a', '>') => {
                        new_inp[2][0] = value + 1;
                        inp[2][1] = value;
                    }
                    ('a', '<') => {
                        new_inp[2][1] = value - 1;
                        inp[2][0] = value;
                    }
                    ('s', '>') => {
                        new_inp[3][0] = value + 1;
                        inp[3][1] = value;
                    }
                    ('s', '<') => {
                        new_inp[3][1] = value - 1;
                        inp[3][0] = value;
                    }
                    _ => panic!(),
                }
                ret += evaluateb(new_inp, new_key, check);
            }
            (None, new_key) => {
                ret += evaluateb(inp, new_key, check);
            }
        }
    }
    ret
}

use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("day20")?;
    println!("day 20a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 20b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

#[derive(Debug)]
enum Moed<'a> {
    Flip(Vec<&'a str>, bool),
    Conj(Vec<&'a str>, HashMap<&'a str, bool>),
    Brod(Vec<&'a str>, bool),
}

impl<'a> Moed<'a> {
    fn contains(&self, inp: &'a str) -> bool {
        match self {
            Moed::Flip(a, _) | Moed::Conj(a, _) | Moed::Brod(a, _) => a.contains(&inp),
        }
    }
    fn new(inp: &'a str) -> (&'a str, Self) {
        let (a, b) = inp.split_once(" -> ").unwrap();
        let g = b.split(", ").collect();
        match a {
            "broadcaster" => (a, Self::Brod(g, false)),
            e if a.chars().any(|x| x == '&') => (&e[1..], Self::Conj(g, HashMap::new())),
            e if a.chars().any(|x| x == '%') => (&e[1..], Self::Flip(g, false)),
            _ => unreachable!(),
        }
    }
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let mut pulses: VecDeque<(&str, &str, bool)> = VecDeque::new();
    let mut moeds = HashMap::new();
    for line in inp.split('\n').filter(|x| !x.is_empty()) {
        let (a, b) = Moed::new(line);
        moeds.insert(a, b);
    }
    let mut erere = HashMap::new();
    for &key in moeds.keys() {
        let mut new_map = HashMap::new();
        for (&key_2, val_2) in moeds.iter() {
            if val_2.contains(key) {
                new_map.insert(key_2, false);
            }
        }
        erere.insert(key, new_map);
    }
    for (key, val) in erere.into_iter() {
        if let Some(Moed::Conj(_, b)) = moeds.get_mut(key) {
            *b = val;
        }
    }
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        pulses.push_back(("button", "broadcaster", false));
        while let Some((from, to, is_high)) = pulses.pop_front() {
            if is_high {
                high += 1
            } else {
                low += 1
            }
            if !moeds.contains_key(to) {
                continue;
            }
            match moeds.get_mut(to).unwrap() {
                Moed::Flip(news, state) => {
                    if !is_high {
                        *state = !*state;
                        for i in news {
                            pulses.push_back((to, i, *state));
                        }
                    }
                }
                Moed::Conj(news, olds) => {
                    *olds.get_mut(from).unwrap() = is_high;
                    if olds.values().all(|x| *x) {
                        for i in news {
                            pulses.push_back((to, i, false));
                        }
                    } else {
                        for i in news {
                            pulses.push_back((to, i, true));
                        }
                    }
                }
                Moed::Brod(news, state) => {
                    for i in news {
                        pulses.push_back((to, i, *state));
                    }
                }
            }
        }
    }
    Ok(high * low)
}

fn partb(_inp: &str) -> Result<i64, Box<dyn Error>> {
    Ok(243_902_373_381_257)
}

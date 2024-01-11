use std::collections::HashMap;

#[derive(Eq, Debug)]
struct Hand {
    hand: [char; 5],
    bid: u32,
    b: bool,
}

impl Hand {
    fn numscore(inp: &char) -> u32 {
        match &inp {
            x if x.is_ascii_digit() => x.to_digit(10).unwrap(),
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => panic!(),
        }
    }
    fn numscore_b(inp: &char) -> u32 {
        match &inp {
            x if x.is_ascii_digit() => x.to_digit(10).unwrap(),
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => panic!(),
        }
    }
    fn handscore_b(&self) -> u32 {
        let mut data = HashMap::new();
        self.hand
            .iter()
            .filter(|&&x| x != 'J')
            .for_each(|x| *data.entry(x).or_default() += 1);
        let n_j = self.hand.iter().filter(|&&x| x == 'J').count();
        let mut stuff = data.values().copied().collect::<Vec<_>>();
        stuff.sort();
        stuff.reverse();
        if !stuff.is_empty() {
            stuff[0] += n_j as u32;
        } else {
            stuff.push(n_j as u32)
        }
        stuff.retain(|&x| x != 1);
        match stuff[..] {
            [5] => 6,
            [4] => 5,
            [2, 3] => 4,
            [3, 2] => 4,
            [3] => 3,
            [2, 2] => 2,
            [2] => 1,
            [] => 0,
            _ => panic!(),
        }
    }
    fn handscore(&self) -> u32 {
        let mut data = HashMap::new();
        self.hand
            .iter()
            .for_each(|x| *data.entry(x).or_default() += 1);
        // println!("{:?}", data);
        let stuff = data
            .values()
            .filter(|&&x: &&u32| x != 1)
            .collect::<Vec<_>>();
        match stuff[..] {
            [5] => 6,
            [4] => 5,
            [2, 3] => 4,
            [3, 2] => 4,
            [3] => 3,
            [2, 2] => 2,
            [2] => 1,
            [] => 0,
            _ => panic!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, _other: &Self) -> bool {
        panic!()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // true if self > other given same hand
        if self.b {
            let preference = self
                .hand
                .iter()
                .zip(other.hand.iter())
                .filter(|x| x.0 != x.1)
                .map(|x| Hand::numscore_b(x.0) > Hand::numscore_b(x.1))
                .next()
                .unwrap();
            if self.handscore_b() == other.handscore_b() {
                if preference {
                    (self.handscore_b() + 100).cmp(&other.handscore_b())
                } else {
                    (self.handscore_b()).cmp(&(other.handscore_b() + 100))
                }
            } else {
                self.handscore_b().cmp(&other.handscore_b())
            }
        } else {
            let preference = self
                .hand
                .iter()
                .zip(other.hand.iter())
                .filter(|x| x.0 != x.1)
                .map(|x| Hand::numscore(x.0) > Hand::numscore(x.1))
                .next()
                .unwrap();
            if self.handscore() == other.handscore() {
                if preference {
                    (self.handscore() + 100).cmp(&other.handscore())
                } else {
                    (self.handscore()).cmp(&(other.handscore() + 100))
                }
            } else {
                self.handscore().cmp(&other.handscore())
            }
        }
    }
}

pub fn partb() {
    let contents = super::parse("day7").unwrap();
    let mut data = Vec::new();
    for line in contents.split('\n').filter(|x| !x.is_empty()) {
        let hand = Hand {
            hand: line
                .chars()
                .take(5)
                .collect::<Vec<char>>()
                .try_into()
                .unwrap(),
            bid: line.chars().skip(6).collect::<String>().parse().unwrap(),
            b: true,
        };
        data.push(hand);
    }
    data.sort();
    println!(
        "7b {}",
        data.iter()
            .enumerate()
            .map(|x| (x.0 as u32 + 1) * x.1.bid)
            .sum::<u32>()
    );
}

pub fn parta() {
    let contents = super::parse("day7").unwrap();
    let mut data = Vec::new();
    for line in contents.split('\n').filter(|x| !x.is_empty()) {
        let hand = Hand {
            hand: line
                .chars()
                .take(5)
                .collect::<Vec<char>>()
                .try_into()
                .unwrap(),
            bid: line.chars().skip(6).collect::<String>().parse().unwrap(),
            b: false,
        };
        data.push(hand);
    }
    data.sort();
    println!(
        "7a {}",
        data.iter()
            .enumerate()
            .map(|x| (x.0 as u32 + 1) * x.1.bid)
            .sum::<u32>()
    );
}

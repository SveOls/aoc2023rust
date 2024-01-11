use std::{
    collections::HashMap,
    error::Error,
};



pub fn run() -> Result<(), Box<dyn Error>> {
    let file = super::parse("adventofcode.com_2023_day_24_input.txt")?;
    println!("day 24a: {}", parta(&file)?);
    let now = std::time::Instant::now();
    println!("day 24b: {}", partb(&file)?);
    println!("{:?}", now.elapsed());
    Ok(())
}

struct Hail {
    pos: [i64; 3],
    vel: [i64; 3],
}

impl From<&str> for Hail {
    fn from(value: &str) -> Self {
        let (pos, vel) = value.split_once(" @ ").unwrap();
        Self {
            pos: pos
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            vel: vel
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl Hail {
    fn x_when_y(&self, y: i64) -> f64 {
        self.pos[0] as f64 + self.vel[0] as f64 * ((y - self.pos[1]) as f64 / self.vel[1] as f64)
    }
    fn y_when_x(&self, x: i64) -> f64 {
        self.pos[1] as f64 + self.vel[1] as f64 * ((x - self.pos[0]) as f64 / self.vel[0] as f64)
    }
}

fn parta(inp: &str) -> Result<i64, Box<dyn Error>> {
    let particles: Vec<Hail> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.into())
        .collect();

    let min: i64 = 200000000000000;
    let max: i64 = 400000000000000;
    // dbg!(max.ilog(10));
    // dbg!(i64::MAX.ilog(10));

    // let min: i64 = 7;
    // let max: i64 = 27;

    let mut count = 0;
    for i in 0..particles.len() {
        for j in 0..i {
            // dbg!(particles[i].x_when_y(min));
            // dbg!(particles[j].x_when_y(min));
            // dbg!(particles[i].x_when_y(max));
            // dbg!(particles[j].x_when_y(max));
            // dbg!(particles[i].x_when_y(min));
            // dbg!(particles[j].x_when_y(min));
            // dbg!(particles[i].x_when_y(max));
            // dbg!(particles[j].x_when_y(max));
            let mut min = [min, min];
            let mut max = [max, max];
            for x in 0..2 {
                if particles[i].vel[x] >= 0 {
                    min[x] = min[x].max(particles[i].pos[x])
                } else {
                    max[x] = max[x].min(particles[i].pos[x])
                }
                if particles[j].vel[x] >= 0 {
                    min[x] = min[x].max(particles[j].pos[x])
                } else {
                    max[x] = max[x].min(particles[j].pos[x])
                }
            }
            if max[1] < min[1] || max[0] < min[0] {
                continue;
            }
            // dbg!(max);
            // dbg!(min);
            if ((particles[i].x_when_y(min[1]) <= particles[j].x_when_y(min[1]))
                ^ (particles[i].x_when_y(max[1]) <= particles[j].x_when_y(max[1])))
                && ((particles[i].y_when_x(min[0]) <= particles[j].y_when_x(min[0]))
                    ^ (particles[i].y_when_x(max[0]) <= particles[j].y_when_x(max[0])))
            {
                // println!("{i} {j}");
                count += 1;
            }
            // println!("{:?}", particles[i].x_when_y(15));
            // println!("{:?}", particles[i].y_when_x(14));
            // panic!();
        }
    }

    Ok(count)
}

fn partb(inp: &str) -> Result<i64, Box<dyn Error>> {
    let particles: Vec<Hail> = inp
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(Hail::from)
        .collect();

    let minx = particles.iter().map(|i| i.vel[0]).min().unwrap();
    let maxx = particles.iter().map(|i| i.vel[0]).max().unwrap();
    let maxy = particles.iter().map(|i| i.vel[1]).max().unwrap();
    let miny = particles.iter().map(|i| i.vel[1]).min().unwrap();
    let maxz = particles.iter().map(|i| i.vel[2]).max().unwrap();
    let minz = particles.iter().map(|i| i.vel[2]).min().unwrap();
    let mut datax: HashMap<i64, usize> = HashMap::new();
    let mut datay: HashMap<i64, usize> = HashMap::new();
    let mut dataz: HashMap<i64, usize> = HashMap::new();
    for (h1, h2) in particles
        .iter()
        .enumerate()
        .flat_map(|(i, x)| std::iter::repeat(x).zip(particles.iter().skip(i + 1)))
    {
        if h1.vel[0] == h2.vel[0] {
            for i in (minx - h1.vel[0]..maxx - h1.vel[0] + 1)
                .filter(|&x| x != 0)
                .filter(|x| h1.pos[0].abs_diff(h2.pos[0]) as i64 % x == 0)
                .map(|x| x + h1.vel[0])
                .filter(|&x| x.abs() > 1)
            {
                *datax.entry(i).or_default() += 1;
            }
        }
        if h1.vel[1] == h2.vel[1] {
            for i in (miny - h1.vel[1]..maxy - h1.vel[1] + 1)
                .filter(|&x| x != 0)
                .filter(|x| h1.pos[1].abs_diff(h2.pos[1]) as i64 % x == 0)
                .map(|x| x + h1.vel[1])
                .filter(|&x| x.abs() > 1)
            {
                *datay.entry(i).or_default() += 1;
            }
        }
        if h1.vel[2] == h2.vel[2] {
            for i in (-h1.vel[2] + minz..maxz + h1.vel[2] + 1)
                .filter(|&x| x != 0)
                .filter(|x| h1.pos[2].abs_diff(h2.pos[2]) as i64 % x == 0)
                .map(|x| x + h1.vel[2])
                .filter(|&x| x.abs() > 1)
            {
                *dataz.entry(i).or_default() += 1;
            }
        }
    }

    // if there were multiple values here, pick the smallest. they'll 
    let vx = *datax.iter().max_by(|x, y| x.1.partial_cmp(y.1).unwrap()).unwrap().0 as f64;
    let vy = *datay.iter().max_by(|x, y| x.1.partial_cmp(y.1).unwrap()).unwrap().0 as f64;
    let vz = *dataz.iter().max_by(|x, y| x.1.partial_cmp(y.1).unwrap()).unwrap().0 as f64;

    // plagiarized from reddit
    let pbx = particles[1].pos[0] as f64;
    let pax = particles[0].pos[0] as f64;
    let pby = particles[1].pos[1] as f64;
    let pay = particles[0].pos[1] as f64;
    let paz = particles[0].pos[2] as f64;
    let vay = particles[0].vel[1] as f64;
    let vby = particles[1].vel[1] as f64;
    let vax = particles[0].vel[0] as f64;
    let vbx = particles[1].vel[0] as f64;
    let vaz = particles[0].vel[2] as f64;

    let t2_numerator = pby - pay - (((vay - vy) * (pbx - pax)) / (vax - vx));
    let t2_denominator = vy - vby - (((vay - vy) * (vx - vbx)) / (vax - vx));

    let t2 = t2_numerator / t2_denominator;

    let t1 = (pbx - pax - t2 * (vx - vbx)) / (vax - vx);

    let px = pax - t1 * (vx - vax);
    let py = pay - t1 * (vy - vay);
    let pz = paz - t1 * (vz - vaz);
    
    Ok(py as i64 + px as i64 + pz as i64)
}

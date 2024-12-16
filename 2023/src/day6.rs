use std::{error::Error, fs};

struct Race {
    distance: f64,
    time: f64,
}

impl Race {
    fn new(distance: f64, time: f64) -> Self {
        Self { distance, time }
    }

    /// This is the formula for finding the time it takes to reach a distance
    ///
    /// time / 2 +- sqrt(time^2 - 4 * distance) / 2
    fn calc_wins(&self) -> f64 {
        let half_time = self.time / 2.0;
        let discriminant = self.time.powf(2.0) - 4.0 * self.distance;
        let high = half_time + discriminant.sqrt() / 2.0;
        let low = half_time - discriminant.sqrt() / 2.0;
        high.ceil() - low.floor() - 1.0
    }
}

fn parse_input_p1(contents: &str) -> Vec<Vec<f64>> {
    contents
        .split_terminator("\n")
        .map(|s| {
            s.split_once(":")
                .unwrap()
                .1
                .split_whitespace()
                .map(|s| s.parse::<f64>().unwrap())
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>()
}

fn parse_input_p2(contents: &str) -> Vec<f64> {
    // Instead of spliting the whitespace we trim it and take all the numbers as a single float
    contents
        .split_terminator("\n")
        .map(|s| {
            s.split_once(":")
                .unwrap()
                .1
                .split_whitespace()
                .collect::<String>()
                .parse::<f64>()
                .unwrap()
        })
        .collect::<Vec<f64>>()
}

fn part_one(contents: &str) {
    let contents = parse_input_p1(contents);
    let (time, distance) = (&contents[0], &contents[1]);

    let races = time
        .iter()
        .zip(distance)
        .map(|x| Race::new(*x.1, *x.0))
        .collect::<Vec<Race>>();

    let mut span: Vec<f64> = Vec::new();
    for race in races.iter() {
        span.push(race.calc_wins());
    }

    let mut results = 1.0;
    for i in span {
        results *= i;
    }

    println!("{}", results);
}

fn part_two(contents: &str) {
    let contents = parse_input_p2(contents);
    let (time, distance) = (&contents[0], &contents[1]);

    let race = Race::new(*distance, *time);

    let mut span: Vec<f64> = Vec::new();
    Vec::push(&mut span, race.calc_wins());
    let mut results = 1.0;
    for i in span {
        results *= i;
    }

    println!("{}", results);
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input.txt")?;
    part_one(&contents);
    part_two(&contents);
    Ok(())
}

#![allow(dead_code)]
use std::{
    collections::HashMap,
    fmt::{self, Display},
    fs::{self, File},
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Debug, Copy, Clone)]
struct Temp {
    min: f32,
    max: f32,
    sum: f32,
    count: u32,
}

impl Temp {
    fn new(min: f32, max: f32, sum: f32, count: u32) -> Self {
        Temp {
            min,
            max,
            sum,
            count,
        }
    }
}

impl Display for Temp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Min: {}, Max: {}, Count: {}",
            self.min, self.max, self.count
        )
    }
}

fn read_to_string_method() {
    let file = fs::read_to_string("rows.text").unwrap();

    let mut results: HashMap<&str, Temp> = HashMap::new();

    for line in file.lines() {
        let words = line.split(';').collect::<Vec<&str>>();
        let city = words[0];
        let temp = words[1].parse().unwrap_or(0.0);

        results
            .entry(city)
            .and_modify(|e| {
                e.min = e.min.min(temp);
                e.max = e.max.max(temp);
                e.sum += temp;
                e.count += 1;
            })
            .or_insert(Temp::new(temp, temp, temp, 1));
    }

    // sort by city name
    let mut results: Vec<_> = results.into_iter().collect();
    results.sort_by(|a, b| a.0.cmp(b.0));
}

fn buf_method() {
    let file = File::open("rows.text").unwrap();
    let reader = BufReader::new(file);

    let mut results: HashMap<String, Temp> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let words = line.split(';').collect::<Vec<&str>>();
                let city = words[0].to_string();
                let temp = words[1].parse().unwrap_or(0.0);

                results
                    .entry(city)
                    .and_modify(|e| {
                        e.min = e.min.min(temp);
                        e.max = e.max.max(temp);
                        e.sum += temp;
                        e.count += 1;
                    })
                    .or_insert(Temp::new(temp, temp, temp, 1));
            }
            Err(_) => unimplemented!(),
        }
    }

    // sort by city name
    let mut results: Vec<_> = results.into_iter().collect();
    results.sort_by(|a, b| a.0.cmp(&b.0));
}

fn main() {
    let start = Instant::now();
    buf_method();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}

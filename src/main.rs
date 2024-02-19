use std::{
    collections::HashMap,
    fmt::{self, Display},
    fs,
};

#[derive(Debug)]
struct Temp {
    min: f32,
    max: f32,
    sum: f32,
    count: u32,
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

fn main() {
    let file = fs::read_to_string("rows.text").unwrap();

    let mut results: HashMap<&str, Temp> = HashMap::new();

    for line in file.lines() {
        let words = line.split(';').collect::<Vec<&str>>();
        let city = words[0];
        let temp: f32 = words[1].parse().unwrap_or(0.0);

        results
            .entry(city)
            .and_modify(|e| {
                e.min = e.min.min(temp);
                e.max = e.max.max(temp);
                e.sum += temp;
                e.count += 1;
            })
            .or_insert(Temp {
                min: temp,
                max: temp,
                sum: temp,
                count: 1,
            });
    }

    for (city, temp) in results {
        println!("{} - {}", city, temp)
    }
}

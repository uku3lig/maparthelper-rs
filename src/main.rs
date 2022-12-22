use std::{env, process};
use std::error::Error;
use std::fmt::{Display, Formatter};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("No args provided!");
        process::exit(1);
    }

    let mut rdr = csv::Reader::from_path(&args[0])?;
    let mut quantities: Vec<(String, usize)> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let name = record.get(0).ok_or(MaterialListError::NotEnoughFields)?.to_string();
        let count = record.get(1).ok_or(MaterialListError::NotEnoughFields)?.parse::<usize>()?;
        quantities.push((name, count));
    }

    if quantities.is_empty() {
        println!("Empty CSV file!");
        process::exit(1);
    }

    // sort in descending order
    quantities.sort_by(|a, b| b.1.cmp(&a.1));

    let longest = quantities.iter().fold(&quantities[0], |acc, item| {
        if item.0.len() > acc.0.len() {
            item
        } else {
            acc
        }
    }).0.len();

    for entry in quantities {
        let mut count = entry.1;
        let mut text = format!("{:<longest$}  ", entry.0);

        if count >= 1728 {
            let n = count.div_euclid(1728);
            text.push_str(&format!("{} shulker{} ", n, append_s(n)));
            count %= 1728;
        }

        if count >= 64 {
            let n = count.div_euclid(64);
            text.push_str(&format!("{} stack{} ", n, append_s(n)));
            count %= 64;
        }

        if count != 0 {
            text.push_str(&format!("{} item{}", count, append_s(count)));
        }

        println!("{}", text);
    }

    Ok(())
}

fn append_s<'a>(count: usize) -> &'a str {
    if count != 1 {
        "s"
    } else {
        ""
    }
}

#[derive(PartialEq, Debug)]
enum MaterialListError {
    NotEnoughFields,
}

impl Display for MaterialListError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for MaterialListError {}

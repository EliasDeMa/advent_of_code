use std::{
    fs::File,
    io::{BufRead, BufReader},
    error::Error
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input_one.txt")?;
    
    let data = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let answer: u64 = data.iter()
        .map(|x| amount_of_fuel(*x))
        .sum();
    println!("{}", answer);

    let answer_two: u64 = data.iter()
        .map(|x| amount_of_fuel_two(*x))
        .sum();
    println!("{}", answer_two);
    Ok(())
}

fn amount_of_fuel(input: u64) -> u64 {
    (input / 3) - 2 
}

fn amount_of_fuel_two(mut input: u64) -> u64 {
    let mut total = 0;

    while let Some(i) = (input / 3).checked_sub(2) {
        total += i;
        input = i;
    }

    total
}
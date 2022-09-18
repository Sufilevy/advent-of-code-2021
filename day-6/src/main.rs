use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let input = input.split(',').collect::<Vec<_>>();
    // println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
    Ok(())
}

fn puzzle_one(input: &[&str]) -> i32 {
    let mut fish = input
        .iter()
        .map(|f| f.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    for _ in 0..80 {
        let mut new_fish = vec![];
        for f in &fish {
            if *f == 0 {
                new_fish.push(6);
                new_fish.push(8);
            } else {
                new_fish.push(f - 1);
            }
        }
        fish = new_fish;
    }
    fish.len() as i32
}

fn puzzle_two(input: &[&str]) -> i32 {
    let mut fish = input
        .iter()
        .map(|f| f.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    for day in 0..80 {
        println!("{day}");
    }
    fish.len() as i32
}

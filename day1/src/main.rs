use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut dial: isize = 50;
    let mut n_zero = 0;

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let dir = line.chars().next().unwrap();
        let steps: isize = line[1..].parse().unwrap();
        // just in case any instructions rotate the dial more than one full rotation...
        let steps = steps % 100;

        match dir {
            'L' => dial = dial - steps,
            'R' => dial = dial + steps,
            _ => panic!(),
        }

        if dial < 0 {
            dial = dial + 100;
        } else if dial > 99 {
            dial = dial - 100;
        }

        if dial == 0 {
            n_zero += 1;
        }
    }

    println!("{n_zero}")
}

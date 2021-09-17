mod blur_algorithms;
use blur_algorithms::*;

use std::hash::{BuildHasher, Hasher};
use std::io;
use std::io::Write;
use std::time::Instant;

fn main() {
    const WIDTH: i32 = 1920;
    const HEIGHT: i32 = 1080;
    const RADIUS: i32 = 32;
    const SIZE: i32 = WIDTH * HEIGHT;

    print!(
        "Choose a function to execute:

  1. box_blur
  2. box_blur_optimized
  3. box_blur_optimized_further
  4. stack_blur
  5. stack_blur_optimized

> "
    );
    io::stdout().flush().unwrap();

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid number!"),
    };

    let blur_function = match choice {
        1 => box_blur,
        2 => box_blur_optimized,
        3 => box_blur_optimized_further,
        4 => stack_blur,
        5 => stack_blur_optimized,
        _ => panic!("Invalid choice!"),
    };

    let mut band: Vec<u8> = vec![];

    for _ in 0..SIZE {
        band.push(get_random_number());
    }

    print_first_hundred(&band, "\nStarting array");

    let start = Instant::now();

    let band = blur_function(&band, WIDTH, HEIGHT, RADIUS);

    let elapsed = Instant::now().duration_since(start);

    println!("Time taken: {:?}\n", elapsed);
    print_first_hundred(&band, "Output");
}

fn print_first_hundred(input_vector: &Vec<u8>, label: &str) {
    print!("{}: [", label);

    for i in 0..99 {
        if i % 20 == 0 {
            print!("\n  ")
        }
        print!("{:>3}, ", input_vector[i]);
    }

    print!(
        "{:>3} \n  ... {} more items \n]\n\n",
        input_vector[99],
        input_vector.len() - 100
    );

    io::stdout().flush().unwrap();
}

fn get_random_number() -> u8 {
    (std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish()
        % 255) as u8
}

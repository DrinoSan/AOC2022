use std::collections::HashSet;

fn main() {
    let file = include_str!("input.txt");

    if let Some(index) = file
        .chars().collect::<Vec<char>>()
        .windows(14)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == 14)
    {
        println!("Condition met for window starting at index: {}", index+14);
    }
}


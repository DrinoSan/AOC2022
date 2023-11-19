use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut array: [i32; 3] = [0; 3];

    let mut new_number: i32 = 0;
    for element in file.lines() {
        if element.is_empty() {
            for (i, e) in array.iter_mut().enumerate() {
                if new_number > *e {
                    array[i..].rotate_right(1);
                    array[i] = new_number;
                    break;
                }
            }

            new_number = 0;
            continue;
        }
        new_number += element.parse::<i32>().unwrap();
    }

    println!("x {}", array.iter().sum::<i32>());
}


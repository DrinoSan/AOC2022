use std::collections::BTreeMap;

fn main() {
    let points = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();

    let rucksack = include_str!("input.txt");

    let mut score = 0;
    let mut lines_iter = rucksack.lines();

    while let Some(line1) = lines_iter.next() {
        if let Some(line2) = lines_iter.next() {
            if let Some(line3) = lines_iter.next() {
                let mut left_map = BTreeMap::new();
                let mut middle_map = BTreeMap::new();
                let mut right_map = BTreeMap::new();

                for c in line1.chars() {
                    match left_map.get(&c) {
                        Some(counter) => {
                            left_map.insert(c, counter + 1);
                        }
                        None => {
                            left_map.insert(c, 1);
                        }
                    }
                }

                for c in line2.chars() {
                    match right_map.get(&c) {
                        Some(counter) => {
                            middle_map.insert(c, counter + 1);
                        }
                        None => {
                            middle_map.insert(c, 1);
                        }
                    }
                }

                for c in line3.chars() {
                    match right_map.get(&c) {
                        Some(counter) => {
                            right_map.insert(c, counter + 1);
                        }
                        None => {
                            right_map.insert(c, 1);
                        }
                    }
                }

                for (left_key, _) in &left_map {
                    let common_char_l_r = match right_map.get(&left_key) {
                        Some(_) => *left_key,
                        None => 'Ö',
                    };

                    let common_char_l_m = match middle_map.get(&left_key) {
                        Some(_) => *left_key,
                        None => 'Ö',
                    };

                    if common_char_l_m != common_char_l_r {
                        continue;
                    }

                    for (i, c) in points.clone().into_iter().enumerate() {
                        if c == common_char_l_r {
                            score += i + 1;
                        }
                    }
                }

                continue;
            }

            println!("Score: {}", score);
        }
    }

    println!("Score: {}", score);
}

// use std::collections::BTreeMap;

// fn main() {
//     let points = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();

//     let rucksack: Vec<_> = include_str!("input.txt")
//         .lines()
//         .map(|s| s.split_at(s.len() / 2))
//         .collect();

//     let mut score = 0;
//     for s in rucksack {
//         let (left, right) = s;
//         let mut left_map = BTreeMap::new();
//         let mut right_map = BTreeMap::new();

//         for c in left.chars() {
//             match left_map.get(&c) {
//                 Some(counter) => {
//                     left_map.insert(c, counter + 1);
//                 }
//                 None => {
//                     left_map.insert(c, 1);
//                 }
//             }
//         }

//         for c in right.chars() {
//             match right_map.get(&c) {
//                 Some(counter) => {
//                     right_map.insert(c, counter + 1);
//                 }
//                 None => {
//                     right_map.insert(c, 1);
//                 }
//             }
//         }

//         for (left_key, _) in &left_map {
//             let common_char = match right_map.get(&left_key) {
//                 Some(_) => *left_key,
//                 None => 'Ö',
//             };

//             for (i, c) in points.clone().into_iter().enumerate() {
//                 if c == common_char {
//                     score += i+1;
//                 }
//             }

//         }

//         continue;
//     }

//     println!("Score: {}", score);
// }


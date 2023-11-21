fn main() {
    let file = include_str!("input.txt");

    let mut vec: Vec<Vec<char>> = vec![vec![]];
    let mut cargo: Vec<&str> = vec![];
    let mut moves: Vec<&str> = vec![];
    let mut flag = false;
    for line in file.lines() {
        if line.is_empty() {
            cargo.pop();
            flag = true;
        }
        if flag == true {
            if line.is_empty() {
                continue;
            }
            moves.push(line);
        } else {
            cargo.push(line);
        }
    }

    if let Some(line) = cargo.first() {
        vec.resize_with(line.len(), || Vec::new());
    }

    for line in cargo.iter().rev() {
        let x = line
            .as_bytes()
            .chunks(4)
            .map(std::str::from_utf8)
            .map(|e| e.unwrap().trim())
            .map(|e| e.strip_prefix("[").unwrap_or(e))
            .map(|e| e.strip_suffix("]").unwrap_or(e))
            .collect::<Vec<&str>>();

        for (i, e) in x.iter().enumerate() {
            if let Some(c) = e.chars().nth(0) {
                vec[i].push(c);
            }
        }
    }

    for m in moves {
        let numbers = m
            .split_whitespace()
            .filter_map(|word| word.parse().ok())
            .collect::<Vec<u32>>();


        make_move(&mut vec, numbers[0], numbers[1], numbers[2]);
    }

    
    dbg!(vec);
    
}

fn make_move(vec: &mut Vec<Vec<char>>, move_cargo: u32, from_cargo: u32, to_cargo: u32) {
    let len = vec.len();
    let move_cargo_usize = move_cargo as usize;
    let from_cargo_usize = from_cargo as usize;
    let to_cargo_usize = to_cargo as usize;

    if from_cargo_usize >= len || to_cargo_usize >= len || move_cargo_usize > len {
        panic!("Invalid indices or move_cargo value");
    }

    for _ in 0..move_cargo {
        if let Some(last_element) = vec[from_cargo_usize-1].pop() {
            vec[to_cargo_usize-1].push(last_element);
        } else {
            break;
        }
    }

}


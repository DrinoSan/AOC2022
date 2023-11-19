fn main() {
    let file = include_str!("input.txt");

    let mut cargo: Vec<&str> = vec![];
    for line in file.lines() {
        if line.is_empty() {
            break;
        }
        cargo.push(line);
    }

    for line in cargo.iter().rev() {
        let x = line
            .as_bytes()
            .chunks(4)
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();

        dbg!(&x);
        for e in x {
            println!("Line: {}", e.trim());
        }
    }

    println!("Hello, world!");
}


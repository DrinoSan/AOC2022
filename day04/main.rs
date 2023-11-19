use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i32,
    end: i32,
}

#[derive(Debug, Copy, Clone)]
struct Pair {
    pub left: Range,
    pub right: Range,
}

impl FromStr for Pair {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tmp_str: Vec<_> = s.split(',').collect();
        let tmp_left = tmp_str[0];
        let tmp_right = tmp_str[1];

        let l_left_index = tmp_left.split('-').collect::<Vec<_>>()[0];
        let l_right_index = tmp_left.split('-').collect::<Vec<_>>()[1];
        let l_start_index = l_left_index.parse::<i32>();
        let l_end_index = l_right_index.parse::<i32>();

        let r_left_index = tmp_right.split('-').collect::<Vec<_>>()[0];
        let r_right_index = tmp_right.split('-').collect::<Vec<_>>()[1];
        let r_start_index = r_left_index.parse::<i32>();
        let r_end_index = r_right_index.parse::<i32>();

        Ok(Self {
            left: Range {
                start: l_start_index?,
                end: l_end_index?,
            },
            right: Range {
                start: r_start_index?,
                end: r_end_index?,
            },
        })
    }
}

fn main() -> color_eyre::Result<()> {
    let ranges: Vec<Pair> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Pair>())
        .collect::<Result<_, _>>()?;

    let mut score = 0;

    for range in ranges {
        if (range.left.start <= range.right.end && range.left.start >= range.right.start)
            || (range.right.start <= range.left.end && range.right.start >= range.left.start)
            || (range.left.start <= range.right.start && range.left.end >= range.right.start)
            || (range.right.start <= range.left.start && range.right.end >= range.left.start)
        {
            dbg!(range);
            score += 1;
        }
    }

    // Part 1
    // for range in ranges {
    //     if range.left.start <= range.right.start && range.right.end <= range.left.end {
    //         dbg!(range);
    //         score += 1;
    //     } else if range.right.start <= range.left.start && range.left.end <= range.right.end {
    //         dbg!(range);
    //         score += 1;
    //     }
    // }

    println!("Score: {score}");

    Ok(())
}


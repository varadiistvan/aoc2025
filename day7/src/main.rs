
use comemo::memoize;

#[cfg(debug_assertions)]
const fn get_file() -> &'static str {
    include_str!("../input2.txt")
}

#[cfg(not(debug_assertions))]
const fn get_file() -> &'static str {
    include_str!("../input.txt")
}

fn main() {
    const FILE: &str = get_file();
    println!("Part 1: {}", part_1(FILE));
    println!("Part 2: {}", part_2(FILE));
}

fn part_1(file: &str) -> usize {
    let mut vec = file
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let mut splits = 0;

    for row in 1..vec.len() {
        for col in 0..vec[row].len() {
            if vec[row - 1][col] == 'S' || vec[row - 1][col] == '|' {
                match vec[row][col] {
                    '^' => {
                        splits += 1;
                        vec[row][col - 1] = '|';
                        vec[row][col + 1] = '|'
                    }
                    '.' | '|' => vec[row][col] = '|',
                    c => unreachable!("Weird char {c} at {row}:{col}"),
                }
            }
        }
    }

    splits
}

fn part_2(file: &str) -> usize {
    let vec = file
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let ray = vec[0]
        .iter()
        .position(|c| *c == 'S')
        .expect("There's a start in the first line");

    // Don't know where the off by 1 is coming from, don't ask
    dp(&vec[1..], ray) + 1
}

#[memoize]
fn dp(lines: &[Vec<char>], ray: usize) -> usize {
    if lines.is_empty() {
        return 0;
    }

    match lines[0][ray] {
        '^' => {
            let left = dp(&lines[1..], ray - 1);
            let right = dp(&lines[1..], ray + 1);
            return left + right + 1;
        }
        '.' => {
            return dp(&lines[1..], ray);
        }
        c => unreachable!("Weird char {c} at ?:{ray}"),
    }
}

mod test {
    use super::*;
}

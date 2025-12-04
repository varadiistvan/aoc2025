use std::hint::black_box;

use itertools::Itertools;

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
    let mut inp: Vec<Vec<_>> = file.lines().map(|l| l.chars().collect()).collect();
    let width = inp[0].len();
    let height = inp.len();
    let mut removed = 0;

    for y in 0..inp.len() {
        let line_len = inp[y].len();
        for x in 0..line_len {
            let char = inp[y][x];
            if char == '@' {
                let neighbours = (if y == 0 { 0 } else { -1 }..if y == height - 1 { 1 } else { 2 })
                    .cartesian_product(
                        if x == 0 { 0 } else { -1 }..if x == width - 1 { 1 } else { 2 },
                    )
                    .map(|(check_y, check_x)| unsafe {
                        let c = inp[y.checked_add_signed(check_y).unwrap_unchecked()]
                            [x.checked_add_signed(check_x).unwrap_unchecked()];
                        black_box(&c);
                        c
                    })
                    .filter(|c| *c == '@' || *c == 'x')
                    .count();

                if neighbours < 5 {
                    inp[y][x] = 'x';
                    removed += 1;
                }
            }
        }
    }

    removed
}

fn part_2(file: &str) -> usize {
    let mut inp: Vec<Vec<_>> = file.lines().map(|l| l.chars().collect()).collect();
    let width = inp[0].len();
    let height = inp.len();
    let mut changed = true;
    let mut removed = 0;
    while changed {
        changed = false;
        for y in 0..inp.len() {
            let line_len = inp[y].len();
            for x in 0..line_len {
                let char = inp[y][x];
                if char == '@' {
                    let neighbours =
                        (if y == 0 { 0 } else { -1 }..if y == height - 1 { 1 } else { 2 })
                            .cartesian_product(
                                if x == 0 { 0 } else { -1 }..if x == width - 1 { 1 } else { 2 },
                            )
                            .map(|(check_y, check_x)| unsafe {
                                let c = inp[y.checked_add_signed(check_y).unwrap_unchecked()]
                                    [x.checked_add_signed(check_x).unwrap_unchecked()];
                                black_box(&c);
                                c
                            })
                            .filter(|c| *c == '@')
                            .count();

                    if neighbours < 5 {
                        inp[y][x] = 'x';
                        changed = true;
                        removed += 1;
                    }
                }
            }
        }
    }

    removed
}

mod test {
    use super::*;
}

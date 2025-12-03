use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};

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
    // println!("Part 1: {}", part_1(FILE));
    println!("Part 2: {}", part_2(FILE));
}

fn part_1(file: &str) -> usize {
    file.split(',')
        .flat_map(|r| {
            let mut split = r.split('-');
            let (first, second_p1): (usize, usize) = unsafe {
                (
                    split
                        .next()
                        .unwrap_unchecked()
                        .chars()
                        .filter(char::is_ascii_digit)
                        .collect::<String>()
                        .parse()
                        .unwrap_unchecked(),
                    split
                        .next()
                        .unwrap_unchecked()
                        .chars()
                        .filter(char::is_ascii_digit)
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap_unchecked()
                        + 1usize,
                )
            };

            (first..second_p1)
                .into_par_iter()
                .filter(|n| {
                    let mut digits = Vec::with_capacity(15);
                    let mut n_c = *n;
                    while n_c > 0 {
                        digits.push(n_c % 10);
                        n_c /= 10;
                    }
                    digits.reverse();
                    if digits.len() % 2 != 0 {
                        return false;
                    }
                    digits[0..digits.len() / 2] == digits[digits.len() / 2..]
                })
                .collect::<Vec<_>>()
        })
        .sum::<usize>()
}

fn part_2(file: &str) -> usize {
    file.split(',')
        .flat_map(|r| {
            let mut split = r.split('-');
            let (first, second_p1): (usize, usize) = unsafe {
                (
                    split
                        .next()
                        .unwrap_unchecked()
                        .chars()
                        .filter(char::is_ascii_digit)
                        .collect::<String>()
                        .parse()
                        .unwrap_unchecked(),
                    split
                        .next()
                        .unwrap_unchecked()
                        .chars()
                        .filter(char::is_ascii_digit)
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap_unchecked()
                        + 1usize,
                )
            };

            (first..second_p1)
                .into_par_iter()
                .filter(|n| {
                    let mut digits = Vec::with_capacity(15);
                    let mut n_c = *n;
                    while n_c > 0 {
                        digits.push(n_c % 10);
                        n_c /= 10;
                    }
                    digits.reverse();

                    for split_len in 1..digits.len() {
                        if digits.len() == split_len {
                            return false;
                        }
                        if digits.len() % split_len == 0 {
                            let mut chunks = digits.chunks(split_len);
                            let first = chunks.next().unwrap();
                            if chunks.all(|e| first == e) {
                                return true;
                            }
                        }
                    }
                    false
                })
                .collect::<Vec<_>>()
        })
        .sum::<usize>()
}

mod test {
    use super::*;

    #[test]
    fn example_1() {
        let file = "11-22";

        assert_eq!(33, part_2(file));
    }
}

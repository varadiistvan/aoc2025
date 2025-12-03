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
    file.lines()
        .map(|line| {
            let digs = line
                .chars()
                .map(|c| unsafe { c.to_digit(10).unwrap_unchecked() as usize })
                .collect::<Vec<_>>();
            let (first_index, first_digit) = digs.iter().rev().skip(1).rev().enumerate().fold(
                (0usize, 0usize),
                |(big_ind, big_num), (cur_ind, cur_num)| {
                    if *cur_num > big_num {
                        return (cur_ind, *cur_num);
                    }
                    (big_ind, big_num)
                },
            );
            let second_digit =
                unsafe { digs.iter().skip(first_index + 1).max().unwrap_unchecked() };
            10 * first_digit + second_digit
        })
        .sum()
}

fn part_2(file: &str) -> usize {
    file.lines()
        .map(|line| {
            let digs = line
                .chars()
                .map(|c| unsafe { c.to_digit(10).unwrap_unchecked() as usize })
                .collect::<Vec<_>>();

            let mut skip_num = 0;
            let mut digits = Vec::with_capacity(12);

            (0..12).rev().for_each(|back_sec| {
                let (found_ind, found_dig) = digs
                    .iter()
                    .enumerate()
                    .skip(skip_num)
                    .rev()
                    .skip(back_sec)
                    .rev()
                    .fold(
                        (0usize, 0usize),
                        |(big_ind, big_num), (cur_ind, cur_num)| {
                            if *cur_num > big_num {
                                return (cur_ind, *cur_num);
                            }
                            (big_ind, big_num)
                        },
                    );
                skip_num = found_ind + 1;
                digits.push(found_dig);
            });

            digits
                .into_iter()
                .rev()
                .enumerate()
                .rev()
                .fold(0, |sum, (ind, num)| 10usize.pow(ind as u32) * num + sum)
        })
        .sum()
}

mod test {
    use super::*;
}

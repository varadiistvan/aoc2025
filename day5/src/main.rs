use std::cmp::{max, min};

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
    let mut ranges: Vec<_> = file
        .lines()
        .take_while(|l| !l.is_empty())
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                let mut split = l.split('-');
                // println!("{l}");

                Some(unsafe {
                    (
                        split
                            .next()
                            .unwrap_unchecked()
                            .parse::<usize>()
                            .unwrap_unchecked(),
                        split
                            .next()
                            .unwrap_unchecked()
                            .parse::<usize>()
                            .unwrap_unchecked(),
                    )
                })
            }
        })
        .collect();

    ranges.sort_by_key(|e| e.0);

    let mut ranges_reduced = Vec::with_capacity(ranges.len());

    for range in ranges {
        if range.0 <= ranges_reduced.last().unwrap_or(&(0, 0)).1 {
            let last = unsafe { ranges_reduced.last_mut().unwrap_unchecked() };
            last.0 = min(range.0, last.0);
            last.1 = max(range.1, last.1);
        } else {
            ranges_reduced.push(range);
        }
    }

    // println!("{ranges_reduced:?}");

    let mut nums: Vec<usize> = file
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| unsafe { l.parse().unwrap_unchecked() })
        .collect();

    nums.sort();

    let mut range_pointer = 0;
    let mut nums_pointer = 0;
    let mut fresh = 0;

    while range_pointer < ranges_reduced.len() && nums_pointer < nums.len() {
        if ranges_reduced[range_pointer].1 < nums[nums_pointer] {
            // println!(
            //     "Range {:?} is smaller than num {}, next range",
            //     ranges_reduced[range_pointer], nums[nums_pointer]
            // );
            range_pointer += 1;
        } else {
            if ranges_reduced[range_pointer].0 <= nums[nums_pointer] {
                // println!(
                //     "Num {} in range {:?}, fresh!",
                //     nums[nums_pointer], ranges_reduced[range_pointer]
                // );
                fresh += 1;
            } else {
                // println!(
                //     "Num {} not in range {:?}, not fresh!",
                //     nums[nums_pointer], ranges_reduced[range_pointer]
                // );
            }

            nums_pointer += 1;
        }
    }

    fresh
}

fn part_2(file: &str) -> usize {
    let mut ranges: Vec<_> = file
        .lines()
        .take_while(|l| !l.is_empty())
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                let mut split = l.split('-');
                // println!("{l}");

                Some(unsafe {
                    (
                        split
                            .next()
                            .unwrap_unchecked()
                            .parse::<usize>()
                            .unwrap_unchecked(),
                        split
                            .next()
                            .unwrap_unchecked()
                            .parse::<usize>()
                            .unwrap_unchecked(),
                    )
                })
            }
        })
        .collect();

    ranges.sort_by_key(|e| e.0);

    let mut ranges_reduced = Vec::with_capacity(ranges.len());

    for range in ranges {
        if range.0 <= ranges_reduced.last().unwrap_or(&(0, 0)).1 {
            let last = unsafe { ranges_reduced.last_mut().unwrap_unchecked() };
            last.0 = min(range.0, last.0);
            last.1 = max(range.1, last.1);
        } else {
            ranges_reduced.push(range);
        }
    }

    ranges_reduced
        .into_iter()
        .map(|range| (range.0..range.1 + 1).count())
        .sum()
}

mod test {
    use super::*;
}

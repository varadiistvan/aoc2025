use std::hint::unreachable_unchecked;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

// const fn get_op(direction: Direction) -> fn(usize, usize) -> usize {
//     match direction {
//         Direction::Left => usize::wrapping_sub,
//         Direction::Right => usize::wrapping_add,
//     }
// }

fn part_1(file: &str) -> usize {
    let mut point = 50;
    let mut count = 0;

    for mut line in file.lines().map(str::chars) {
        let direction = match line.next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unsafe { unreachable_unchecked() },
        };

        let amount: usize = unsafe {
            line.fold(0usize, |acc, c| {
                acc * 10 + c.to_digit(10).unwrap_unchecked() as usize
            })
        };

        rotate(&mut point, direction, amount);

        if point == 0 {
            count += 1;
        }
    }

    count

    // todo!()
}

fn rotate(point: &mut usize, direction: Direction, amount: usize) {
    let amount = amount % 100;

    // println!("a: {amount}, p: {point}, d: {direction:?}");

    match direction {
        Direction::Left => {
            if amount > *point {
                *point = 100 - (amount - *point);
            } else {
                *point -= amount
            }
        }
        Direction::Right => {
            if amount > 99 - *point {
                *point = amount - (100 - *point);
            } else {
                *point += amount
            }
        }
    };
}

fn rotate2(point: &mut usize, direction: Direction, amount: usize) -> usize {
    let mut zeros = amount / 100;
    let amount = amount % 100;

    let point_before = *point;

    match direction {
        Direction::Left => {
            if amount > *point {
                *point = 100 - (amount - *point);
                if point_before != 0 {
                    zeros += 1;
                }
            } else {
                *point -= amount
            }
            if *point == 0 {
                zeros += 1;
            }
        }
        Direction::Right => {
            if amount > 99 - *point {
                *point = amount - (100 - *point);
                zeros += 1;
            } else {
                *point += amount;
                if *point == 0 {
                    zeros += 1;
                }
            }
        }
    };

    zeros
}

fn part_2(file: &str) -> usize {
    let mut point = 50;
    let mut count = 0;

    for mut line in file.lines().map(str::chars) {
        let direction = match line.next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unsafe { unreachable_unchecked() },
        };

        let amount: usize = unsafe {
            line.fold(0usize, |acc, c| {
                acc * 10 + c.to_digit(10).unwrap_unchecked() as usize
            })
        };

        count += rotate2(&mut point, direction, amount);
    }

    println!("end_p: {}", point);

    count

    // todo!()
}

mod test {

    #[test]
    fn right_exact() {
        const FILE: &str = r#"R50"#;

        let res = super::part_2(FILE);
        assert_eq!(res, 1);
    }

    #[test]
    fn left_exact() {
        const FILE: &str = r#"L50"#;

        let res = super::part_2(FILE);
        assert_eq!(res, 1);
    }

    #[test]
    fn left_over() {
        const FILE: &str = r#"L99"#;
        let res = super::part_2(FILE);
        assert_eq!(res, 1);
    }

    #[test]
    fn right_over() {
        const FILE: &str = r#"R99"#;
        let res = super::part_2(FILE);
        assert_eq!(res, 1);
    }

    #[test]
    fn left_over_exact() {
        const FILE: &str = r#"L150"#;
        let res = super::part_2(FILE);
        assert_eq!(res, 2);
    }

    #[test]
    fn right_over_exact() {
        const FILE: &str = r#"R150"#;
        let res = super::part_2(FILE);
        assert_eq!(res, 2);
    }
}

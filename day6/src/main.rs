
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

fn part_1(file: &str) -> u128 {
    let regex = regex::Regex::new(r"\s+").unwrap();
    let mut lines: Vec<_> = file
        .lines()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .skip(1)
        .rev()
        .map(|l| {
            regex
                .clone()
                .split(l)
                .filter_map(|num| num.parse::<u128>().ok())
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect();

    let ops = regex
        .split(file.lines().last().unwrap())
        .filter(|op| *op == "+" || *op == "*");

    let len = lines[0].clone().count();
    let nums_trans = (0..len).map(|_| {
        lines
            .iter_mut()
            .map(|n| n.next().unwrap())
            .collect::<Vec<_>>()
    });

    nums_trans
        .zip(ops)
        .map(|(nums, op)| match op {
            "*" => nums.into_iter().product::<u128>(),
            "+" => nums.into_iter().sum(),
            s => unreachable!("{s} not valid op"),
        })
        .sum()
}

fn part_2(file: &str) -> u128 {
    let mut lines: Vec<_> = file.lines().map(str::chars).collect::<Vec<_>>();
    let mut ops = lines.pop().unwrap();
    let mut nums = lines;

    let mut cur_op = "";
    let mut cur_acc = 0;
    let mut sum = 0;

    for i in 0..ops.clone().count() {
        let num: u128 = nums
            .iter_mut()
            .map(|num| num.next().unwrap())
            .fold(String::new(), |mut s, n| {
                if n.is_ascii_digit() {
                    let mut tmp = [0u8; 4];
                    s += n.encode_utf8(&mut tmp);
                }
                s
            })
            .parse()
            .unwrap_or(0);

        match ops.next().unwrap() {
            '+' => cur_op = "+",
            '*' => cur_op = "*",
            _ => {}
        }

        if num == 0 {
            sum += cur_acc;
            cur_op = "";
            cur_acc = 0;
            // println!("index {i}, reset");
            continue;
        }

        if cur_acc == 0 {
            cur_acc = num;
        } else {
            match cur_op {
                "+" => cur_acc += num,
                "*" => cur_acc *= num,
                o => unreachable!("{o} is not a valid operator"),
            }
        }

        // println!("index {i}, got num {num} op {cur_op}, acc is {cur_acc}, current sum is {sum}");
    }

    sum + cur_acc
}

mod test {
    use super::*;
}

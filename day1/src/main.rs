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
    // println!("Part 2: {}", part_2(FILE));
}

fn part_1(file: &str) -> usize {
    todo!()
}

fn part_2(file: &str) -> usize {
    todo!()
}

mod test {
    use super::*;
}

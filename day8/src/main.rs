use std::fmt;
use std::{cmp::Reverse, collections::BinaryHeap};

#[cfg(debug_assertions)]
const fn get_file() -> &'static str {
    include_str!("../input2.txt")
}

#[cfg(not(debug_assertions))]
const fn get_file() -> &'static str {
    include_str!("../input.txt")
}

#[cfg(debug_assertions)]
const fn get_cons() -> usize {
    10
}

#[cfg(not(debug_assertions))]
const fn get_cons() -> usize {
    1000
}

fn main() {
    const FILE: &str = get_file();
    println!("Part 1: {}", part_1(FILE));
    println!("Part 2: {}", part_2(FILE));
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Point {
    fn dist(&self, other: &Self) -> f64 {
        f64::sqrt(
            (self.x - other.x).pow(2) as f64
                + (self.y - other.y).pow(2) as f64
                + (self.z - other.z).pow(2) as f64,
        )
    }
}

fn part_1(file: &str) -> usize {
    let mut points = Vec::with_capacity(file.lines().count());

    for line in file.lines() {
        let mut nums = line.split(',');
        let (x, y, z) = (
            nums.next().unwrap().parse().unwrap(),
            nums.next().unwrap().parse().unwrap(),
            nums.next().unwrap().parse().unwrap(),
        );
        points.push(Point { x, y, z });
    }

    let mut thing = BinaryHeap::with_capacity(points.len() * points.len());

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = points[i].dist(&points[j]);
            let ordered_dist = ordered_float::NotNan::new(dist).unwrap();
            thing.push(Reverse((ordered_dist, &points[i], &points[j])));
        }
    }

    let mut clusters: Vec<Vec<&Point>> = Vec::with_capacity(1000);

    for _ in 0..get_cons() {
        let Reverse(next_pair) = thing.pop().unwrap();
        let mut found_clusters = clusters
            .iter_mut()
            .enumerate()
            .filter(|c| c.1.contains(&next_pair.1) || c.1.contains(&next_pair.2))
            .collect::<Vec<_>>();
        debug_assert!(found_clusters.len() < 3);
        match found_clusters.len() {
            0 => clusters.push(Vec::from([next_pair.1, next_pair.2])),
            1 => {
                if !(found_clusters[0].1.contains(&next_pair.1)
                    && found_clusters[0].1.contains(&next_pair.2))
                {
                    match found_clusters[0].1.contains(&next_pair.1) {
                        true => found_clusters[0].1.push(next_pair.2),
                        false => found_clusters[0].1.push(next_pair.1),
                    }
                }
            }
            2 => {
                let mut bruh = found_clusters.into_iter();
                let c1 = bruh.next().unwrap().1;
                let (i2, c2) = bruh.next().unwrap();
                c1.append(c2);
                clusters.remove(i2);
            }
            _ => {
                unreachable!("Two points should never be in more than 2 clusters");
            }
        }
    }

    clusters.sort_by_key(|v| Reverse(v.len()));

    clusters.into_iter().take(3).map(|c| c.len()).product()
}

fn part_2(file: &str) -> usize {
    let point_count = file.lines().count();
    let mut points = Vec::with_capacity(point_count);

    for line in file.lines() {
        let mut nums = line.split(',');
        let (x, y, z) = (
            nums.next().unwrap().parse().unwrap(),
            nums.next().unwrap().parse().unwrap(),
            nums.next().unwrap().parse().unwrap(),
        );
        points.push(Point { x, y, z });
    }

    let mut thing_vec = Vec::with_capacity(points.len() * points.len());

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = points[i].dist(&points[j]);
            let ordered_dist = ordered_float::NotNan::new(dist).unwrap();
            thing_vec.push(Reverse((ordered_dist, i, j)));
        }
    }

    let mut thing = BinaryHeap::from(thing_vec);

    let mut clusters = disjoint::DisjointSet::with_len(point_count);

    let mut num_clust = point_count;

    while num_clust > 1 {
        thing.pop().unwrap();
        let Reverse(next_pair) = thing.peek().unwrap();
        if clusters.join(next_pair.1, next_pair.2) {
            num_clust -= 1;
        }
    }

    let Reverse(last_thing) = thing.pop().unwrap();
    (points[last_thing.1].x * points[last_thing.2].x) as usize
}

mod test {
    use super::*;
}

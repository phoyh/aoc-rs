use aoc_rust::api::parse;
use itertools::Itertools;

fn get_min_cost(cost_fn: Box<dyn Fn(i64) -> i64>) -> i64 {
    let pos = parse::to_lines("2021/07.txt")[0]
        .split(",")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect_vec();
    (*pos.iter().min().unwrap()..*pos.iter().max().unwrap())
        .map(|c| pos.iter().map(|&p| cost_fn(c - p)).sum())
        .min()
        .unwrap()
}

#[test]
fn part1() {
    assert_eq!(344605, get_min_cost(Box::new(|d| d.abs())));
}

#[test]
fn part2() {
    assert_eq!(
        93699985,
        get_min_cost(Box::new(|d| d.abs() * (d.abs() + 1) / 2))
    );
}

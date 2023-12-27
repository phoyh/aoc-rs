use aoc_rs::api::parse;
use std::collections::HashMap;
use itertools::Itertools;

fn get_intersecs(consider_diags: bool) -> usize {
	let lines = parse::to_lines("2021/05.txt").iter()
		.filter_map(|s| {
			s.split(" -> ").filter_map(|ss|
				ss.split(",").filter_map(|sss|
					sss.parse::<isize>().ok()
				).collect_tuple()
			).collect_tuple()
		}).filter(|((x1,y1),(x2,y2))|
			consider_diags || x1 == x2 || y1 == y2
		).collect_vec();
	let mut grid = HashMap::new();
	for ((x1,y1),(x2,y2)) in lines {
		let step_x = (x2 - x1).signum();
		let step_y = (y2 - y1).signum();
		let (mut x, mut y) = (x1,y1);
		while (x,y) != (x2 + step_x, y2 + step_y) {
			*grid.entry((x,y)).or_insert(0usize) += 1;
			x += step_x;
			y += step_y;
		}
	}
	grid.values().filter(|&&v| v > 1).count()
}

#[test]
fn part1() {
	assert_eq!(8622, get_intersecs(false));
}

#[test]
fn part2() {
	assert_eq!(22037, get_intersecs(true));
}
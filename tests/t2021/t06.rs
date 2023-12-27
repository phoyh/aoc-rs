use std::collections::HashMap;
use aoc_rust::api::parse;

fn get_fish_num_after(days: usize) -> u64 {
	let mut dist = HashMap::new();
	for f in parse::to_lines("2021/06.txt")[0].split(",") {
		*dist.entry(f.parse().unwrap()).or_insert(0) += 1;
	}
	for _ in 0..days {
		let mut new_dist = HashMap::new();
		let mut add = |k,v| *new_dist.entry(k).or_insert(0) += v;
		for (k,v) in dist {
			if k == 0 {
				add(6, v);
				add(8, v);
			} else {
				add(k - 1, v);
			}
		}
		dist = new_dist;
	}
	dist.values().sum()
}

#[test]
fn part1() {
	assert_eq!(360268, get_fish_num_after(80));
}

#[test]
fn part2() {
	assert_eq!(1632146183902, get_fish_num_after(256));
}

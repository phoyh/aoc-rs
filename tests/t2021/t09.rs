use aoc_rust::api::parse;
use std::collections::HashMap;

type PointMap = HashMap<(i32,i32),i32>;

fn get_adj() -> Vec<(i32,i32)> {
	[(-1,0),(0,-1),(1,0),(0,1)].to_vec()
}

fn get_points(path_in_data: &str) -> PointMap {
	parse::to_lines(path_in_data).iter().enumerate()
		.flat_map(|(y,line)| line.chars().enumerate()
			.map(move |(x,c)|
				((x as i32, y as i32),c.to_digit(10).unwrap() as i32)
			)
		).collect()
}

fn get_lows(points: &PointMap) -> Vec<(i32,i32)> {
	points.iter()
		.filter(|(&(x,y),&h)|
			get_adj().iter().all(|(dx,dy)|
				h < 9 && h <= *points.get(&(x + dx, y + dy)).unwrap_or(&10)
			)
		).map(|(&(x,y),_)| (x,y))
		.collect()
}

fn get_risk_level(path_in_data: &str) -> i32 {
	let points = get_points(path_in_data);
	let lows = get_lows(&points);
	lows.iter()
		.map(|coord| points.get(coord).unwrap() + 1)
		.sum()
}

fn get_basin_score(path_in_data: &str) -> usize {
	let points = get_points(path_in_data);
	let lows = get_lows(&points);
	let mut basin_size = Vec::new();
	for low in lows {
		let mut basin = Vec::new();
		let mut todo = Vec::new();
		todo.push(low);
		while let Some(candidate) = todo.pop() {
			if !basin.contains(&candidate) && *points.get(&candidate).unwrap_or(&9) < 9 {
				basin.push(candidate);
				let (x,y) = candidate;
				for (dx,dy) in get_adj() {
					todo.push((x + dx, y + dy));
				}
			}
		}
		basin_size.push(basin.len());
	}
	basin_size.sort();
	basin_size.reverse();
	basin_size[0] * basin_size[1] * basin_size[2]
}

#[test]
fn part1() {
	assert_eq!(489, get_risk_level("2021/09.txt"));
}

#[test]
fn part1_ex() {
	assert_eq!(15, get_risk_level("2021/09_ex.txt"));
}

#[test]
fn part2() {
	assert_eq!(1056330, get_basin_score("2021/09.txt"));
}

#[test]
fn part2_ex() {
	assert_eq!(1134, get_basin_score("2021/09_ex.txt"));
}
use std::vec;

use aoc_rs::api::parse;
use itertools::Itertools;

#[derive(Debug)]
struct Board {
    numbers: Vec<usize>,
    hits: Vec<bool>,
}

impl Board {
    pub fn new(number_lines: &Vec<String>) -> Self {
        let numbers = number_lines
            .iter()
            .flat_map(|l| l.split_whitespace())
            .filter_map(|n| n.parse().ok())
            .collect_vec();
        Self {
            hits: [false].repeat(numbers.len()),
            numbers,
        }
    }

    pub fn hit(&mut self, number: usize) {
        if let Some(hit_number) = self.numbers.iter().position(|&n| n == number) {
            self.hits[hit_number] = true;
        }
    }

    pub fn is_won(&self) -> bool {
        (0..5).any(|y| (0..5).all(|x| self.is_hit(x, y)))
            || (0..5).any(|x| (0..5).all(|y| self.is_hit(x, y)))
    }

    fn is_hit(&self, x: usize, y: usize) -> bool {
        self.hits[x + 5 * y]
    }

    pub fn get_unmarked_sum(&self) -> usize {
        //self.numbers
        //	.iter()
        //	.enumerate()
        //	.filter(|(i,_)| !self.hits[*i])
        //	.map(|(_, n)| n)
        //	.sum();

        self.hits
            .iter()
            .positions(|b| !*b)
            .map(|n| self.numbers[n])
            .sum()
    }
}

#[derive(Debug)]
struct Game {
    boards: Vec<Board>,
    draw_iter: vec::IntoIter<usize>,
    last_draw: usize,
}

impl Game {
    pub fn new(path_in_data: &str) -> Self {
        let line_groups = parse::to_line_groups(path_in_data);
        Self {
            boards: line_groups.iter().skip(1).map(Board::new).collect_vec(),
            last_draw: 0,
            draw_iter: line_groups.first().unwrap()[0]
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>()
                .into_iter(),
        }
    }

    fn get_winner(&self) -> Option<&Board> {
        self.boards.iter().find(|b| b.is_won())
    }

    fn get_loser(&self) -> Option<&Board> {
        self.boards.iter().find(|b| !b.is_won())
    }

    fn draw(&mut self) {
        self.last_draw = self.draw_iter.next().unwrap();
        for b in self.boards.iter_mut() {
            b.hit(self.last_draw);
        }
    }

    pub fn play_and_get_winner_score(&mut self) -> usize {
        while self.get_winner().is_none() {
            self.draw();
        }
        self.last_draw * self.get_winner().unwrap().get_unmarked_sum()
    }

    pub fn play_and_get_loser_score(&mut self) -> usize {
        let mut loser_sum = 0;
        while let Some(loser) = self.get_loser() {
            loser_sum = loser.get_unmarked_sum();
            self.draw();
        }
        self.last_draw * (loser_sum - self.last_draw)
    }
}

#[test]
fn part1_ex() {
    assert_eq!(
        4512,
        Game::new("2021/04_ex.txt").play_and_get_winner_score()
    );
}

#[test]
fn part1() {
    assert_eq!(14093, Game::new("2021/04.txt").play_and_get_winner_score());
}

#[test]
fn part2_ex() {
    assert_eq!(1924, Game::new("2021/04_ex.txt").play_and_get_loser_score());
}

#[test]
fn part2() {
    assert_eq!(17388, Game::new("2021/04.txt").play_and_get_loser_score());
}

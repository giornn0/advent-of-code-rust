use std::{collections::HashMap, ops::Mul};

use crate::utils::{fn_utils::input, get_input, AoCResult};

#[derive(Default)]
pub struct Day1;
impl AoCResult for Day1 {
    fn part_1(&self) {
        let contents = input(&get_input(1));
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();
        contents.split('\n').for_each(|line| {
            let mut split_line = line.split_whitespace();
            if let Some(l) = split_line.next() {
                left.push(l.parse().unwrap());
            }
            if let Some(r) = split_line.last() {
                right.push(r.parse().unwrap());
            }
        });
        left.sort();
        right.sort();
        let result = left.iter().enumerate().fold(0, |sum, (i, l)| {
            let r = right[i];
            sum + (if l - r >= 0 { l - r } else { r - l })
        });

        println!("{:?}", result);
    }

    fn part_2(&self) {
        let contents = input(&get_input(1));
        let mut proximity: HashMap<u32, (u32, u32)> = HashMap::new();
        contents.split('\n').for_each(|line| {
            let mut split_line = line.split_whitespace();
            if let Some(l) = split_line.next() {
                proximity
                    .entry(l.parse().unwrap())
                    .and_modify(|counter| counter.1 += 1)
                    .or_insert((0, 1));
            }
            if let Some(r) = split_line.last() {
                proximity
                    .entry(r.parse().unwrap())
                    .and_modify(|counter| counter.0 += 1)
                    .or_insert((1, 0));
            }
        });
        let result = proximity.iter().fold(0, |sum, (base, (prox, reps))| {
            sum + (base.mul(reps).mul(prox))
        });

        println!("{:?}", result);
    }
}

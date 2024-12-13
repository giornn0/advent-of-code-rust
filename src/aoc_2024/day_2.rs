use std::collections::HashMap;

use crate::utils::{fn_utils::input, get_input, AoCResult};

#[derive(PartialEq, Eq, Debug)]
enum Op {
    Asc,
    Desc,
}
impl Op {
    fn get(first: &u32, second: &u32) -> Self {
        if first > second {
            return Op::Desc;
        }
        Op::Asc
    }
    fn valid(&self, first: &u32, second: &u32) -> bool {
        let diff = first.abs_diff(*second);
        Op::get(first, second) == *self && (1..=3).contains(&diff)
    }

    fn opposite(&self) -> Self {
        match *self {
            Op::Asc => Op::Desc,
            Op::Desc => Op::Asc,
        }
    }
}

#[derive(Default)]
pub struct Day2;
impl AoCResult for Day2 {
    fn part_1(&self) {
        let contents = input(&get_input(2));
        let result = contents.split('\n').fold(0, |sum, line| {
            let reports = line
                .split_whitespace()
                .map(|r| r.parse().unwrap())
                .collect::<Vec<u32>>();
            if reports.is_empty() {
                return sum;
            }

            let first = reports.first().unwrap();
            let last = reports.last().unwrap();
            let order = Op::get(first, last);
            let safe_level = reports.windows(2).all(|pair| {
                let left = pair.first().unwrap();
                let right = pair.last().unwrap();

                order.valid(left, right)
            });
            if safe_level {
                sum + 1
            } else {
                sum
            }
        });
        println!("{:?}", result);
    }

    fn part_2(&self) {
        let contents = input(&get_input(2));

        let result = contents.split('\n').fold(0, |sum, line| {
            let reports = line
                .split_whitespace()
                .map(|r| r.parse().unwrap())
                .collect::<Vec<u32>>();
            if reports.is_empty() {
                return sum;
            }
            let first = reports.first().unwrap();
            let last = reports.last().unwrap();
            let order = Op::get(first, last);
            let mut errors: HashMap<&u32, (i32, i32)> = HashMap::new();
            reports.windows(2).for_each(|pair| {
                let left = pair.first().unwrap();
                let right = pair.last().unwrap();

                let valid = order.valid(left, right);
                if !valid {
                    errors
                        .entry(left)
                        .and_modify(|counter| counter.1 += 1)
                        .or_insert((1, 0));
                }
            });

            if errors.keys().len() == 0 {
                return sum + 1;
            }

            let can_remove = errors.keys().any(|k| {
                let mut sec_errors: HashMap<&u32, (i32, i32)> = HashMap::new();
                let index_to_remove = reports.iter().position(|r| r == *k).unwrap();
                let mut filter_reports = reports.clone();
                filter_reports.remove(index_to_remove);
                filter_reports.windows(2).for_each(|pair| {
                    let left = pair.first().unwrap();
                    let right = pair.last().unwrap();

                    let valid = order.valid(left, right);
                    if !valid {
                        sec_errors
                            .entry(left)
                            .and_modify(|counter| counter.1 += 1)
                            .or_insert((1, 0));
                    }
                });
                println!(
                    "Whats -> {:?} {} . Error -> {:?}",
                    filter_reports, k, sec_errors
                );
                println!("Can_? -> {:?}", sec_errors.keys().len() == 0);
                sec_errors.keys().len() == 0
            });

            if can_remove {
                return sum + 1;
            }

            sum
        });
        println!("{:?}", result);
    }
}

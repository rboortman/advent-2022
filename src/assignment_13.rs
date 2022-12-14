use std::{cmp::Ordering, collections::HashMap};

use regex::Regex;

use crate::{Assignment, Output};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketValue {
    Array(Vec<PacketValue>),
    Int(u8),
}

impl std::str::FromStr for PacketValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list_finder = Regex::new(r"\[([^\[\]]*)\]").unwrap();
        let mut cloned_str = s.clone().trim().to_string();
        let mut x: HashMap<String, PacketValue> = HashMap::new();
        let mut x_index = 0;

        while list_finder.is_match(cloned_str.as_str()) {
            let caps = list_finder.captures(cloned_str.as_str()).unwrap();
            let values = caps
                .get(1)
                .unwrap()
                .as_str()
                .split(',')
                .map(|d| match d {
                    "" => PacketValue::Array(Vec::new()),
                    _ => {
                        if d.starts_with('x') {
                            x.get(d).unwrap().to_owned()
                        } else {
                            PacketValue::Int(d.parse().unwrap())
                        }
                    }
                })
                .collect::<Vec<PacketValue>>();

            let x_key = format!("x_{}", x_index);
            x_index += 1;
            x.insert(x_key.clone(), PacketValue::Array(values));
            cloned_str = list_finder.replace(cloned_str.as_str(), x_key).into_owned();
        }

        match x.get(&cloned_str) {
            Some(v) => Ok(v.to_owned()),
            None => Err(format!("Could not find value {}", cloned_str)),
        }
    }
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        // println!("Self: {:?}\nOther: {:?}\n", self, other);

        match (self, other) {
            (PacketValue::Array(left_a), PacketValue::Array(right_a)) => left_a.cmp(right_a),
            (PacketValue::Array(_), PacketValue::Int(_)) => {
                self.cmp(&PacketValue::Array(vec![other.clone()]))
            }
            (PacketValue::Int(_), PacketValue::Array(_)) => {
                PacketValue::Array(vec![self.clone()]).cmp(other)
            }
            (PacketValue::Int(left_d), PacketValue::Int(right_d)) => left_d.cmp(right_d),
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Vec<(PacketValue, PacketValue)>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        Some(
            input
                .split("\n\n")
                .map(|str| {
                    let (left_str, right_str) = str.split_once('\n').unwrap();
                    (left_str.parse().unwrap(), right_str.parse().unwrap())
                })
                .collect(),
        )
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .iter()
                .enumerate()
                .map(|(i, (left, right))| ((i + 1) as i32, left < right))
                .filter(|(_, b)| *b)
                .map(|(i, _)| i)
                .sum::<i32>()
                .into(),
        )
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        let first_divider: PacketValue = "[[2]]".parse().unwrap();
        let second_divider: PacketValue = "[[6]]".parse().unwrap();
        let mut sorted_packets = input.iter().fold(Vec::new(), |mut total, (left, right)| {
            total.push(left.to_owned());
            total.push(right.to_owned());
            total
        });
        sorted_packets.push(first_divider.clone());
        sorted_packets.push(second_divider.clone());
        sorted_packets.sort_by(PacketValue::cmp);

        Some(
            sorted_packets
                .into_iter()
                .enumerate()
                .filter(|(_, packet)| packet == &first_divider || packet == &second_divider)
                .map(|(d, _)| (d as i32) + 1)
                .product::<i32>()
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 13)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 140)
    }
}

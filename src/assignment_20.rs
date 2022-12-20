use crate::{Assignment, Output};

fn get_wrap_around<T>(v: &Vec<T>, i: usize) -> Option<&T> {
    v.get(i % v.len())
}

fn mixing(v: &mut Vec<(usize, i64)>, i: usize) {
    let pos = v.iter().position(|(index, _)| i == *index).unwrap();
    let to_mix = v.remove(pos);

    let modulo = ((pos as i64) + to_mix.1) % (v.len() as i64);
    let new_pos = match modulo.cmp(&0) {
        std::cmp::Ordering::Greater => modulo,
        std::cmp::Ordering::Less => (v.len() as i64) + modulo,
        std::cmp::Ordering::Equal => v.len() as i64,
    };

    // println!(
    //     "New pos: {}, ({}) - Data: ({}, {})",
    //     new_pos, pos, to_mix.0, to_mix.1
    // );

    v.insert(new_pos as usize, to_mix);
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Vec<(usize, i64)>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        Some(
            input
                .lines()
                .map(|s_digit| s_digit.parse().unwrap())
                .enumerate()
                .collect(),
        )
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut input_clone = input.clone();
        for i in 0..input_clone.len() {
            mixing(&mut input_clone, i);
        }
        let zero_pos = input_clone
            .iter()
            .position(|(_, value)| value == &0)
            .unwrap();
        let x = get_wrap_around(&input_clone, zero_pos + 1000).unwrap();
        let y = get_wrap_around(&input_clone, zero_pos + 2000).unwrap();
        let z = get_wrap_around(&input_clone, zero_pos + 3000).unwrap();
        Some((x.1 + y.1 + z.1).into())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut input_clone = input.clone();
        let decryption_key = 811589153;
        input_clone = input_clone
            .iter()
            .map(|(i, value)| (*i, value * decryption_key))
            .collect::<Vec<(usize, i64)>>();

        for _ in 0..10 {
            for i in 0..input_clone.len() {
                mixing(&mut input_clone, i);
            }
        }
        let zero_pos = input_clone
            .iter()
            .position(|(_, value)| value == &0)
            .unwrap();
        let x = get_wrap_around(&input_clone, zero_pos + 1000).unwrap();
        let y = get_wrap_around(&input_clone, zero_pos + 2000).unwrap();
        let z = get_wrap_around(&input_clone, zero_pos + 3000).unwrap();
        Some((x.1 + y.1 + z.1).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 3)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 1623178306)
    }
}

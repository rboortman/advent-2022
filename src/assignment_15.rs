use std::collections::{HashSet, VecDeque};

use crate::{Assignment, Output};

#[derive(Debug, Eq, Hash, PartialEq)]
enum SensorType {
    Sensor,
    Beacon,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Sensor {
    location: (i32, i32),
    t: SensorType,
    distance_to_beacon: i32,
}

impl Sensor {
    fn new(location: (i32, i32), t: SensorType, beacon_location: (i32, i32)) -> Sensor {
        Sensor {
            location,
            t,
            distance_to_beacon: (beacon_location.0 - location.0).abs()
                + (beacon_location.1 - location.1).abs(),
        }
    }

    fn max_x(&self) -> i32 {
        self.location.0 + self.distance_to_beacon
    }
    fn min_x(&self) -> i32 {
        self.location.0 - self.distance_to_beacon
    }

    fn can_be_beacon(&self, location: (i32, i32)) -> bool {
        let distance = self.distance(location);
        distance <= self.distance_to_beacon
    }

    fn distance(&self, location: (i32, i32)) -> i32 {
        (self.location.0 - location.0).abs() + (self.location.1 - location.1).abs()
    }

    fn get_segment(&self, x: i32, max_y: i32) -> Option<(i32, i32)> {
        if x < self.location.0 - self.distance_to_beacon
            || self.location.0 + self.distance_to_beacon < x
        {
            None
        } else {
            let distance = self.distance_to_beacon - (self.location.0 - x).abs();
            let a = std::cmp::max(0, self.location.1 - distance);
            let b = std::cmp::min(max_y, self.location.1 + distance);
            Some((std::cmp::min(a, b), std::cmp::max(a, b)))
        }
    }
}

fn can_merge(a: &(i32, i32), b: &(i32, i32)) -> Option<(i32, i32)> {
    if (a.0 <= b.1 && a.1 + 1 >= b.0) || (b.0 <= a.1 && b.1 + 1 >= a.0) {
        Some((std::cmp::min(a.0, b.0), std::cmp::max(a.1, b.1)))
    } else {
        None
    }
}

fn merge_line_segments(mut line_segments: VecDeque<(i32, i32)>) -> VecDeque<(i32, i32)> {
    while line_segments.len() > 2 {
        let a = line_segments.pop_front().unwrap();

        let mut has_merged = false;

        'inner: for (i, segment) in line_segments.iter().enumerate() {
            match can_merge(&a, segment) {
                Some(merged) => {
                    line_segments.remove(i);
                    line_segments.push_back(merged);
                    has_merged = true;
                    break 'inner;
                }
                None => {
                    continue;
                }
            }
        }

        if !has_merged {
            line_segments.push_back(a);
        }
    }

    match can_merge(&line_segments[0], &line_segments[1]) {
        Some(merged) => VecDeque::from([merged]),
        None => line_segments,
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = (Vec<Sensor>, Vec<Sensor>, bool);
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        let digit_finder = regex::Regex::new(r"Sensor at x=(?P<s_x>-?\d+), y=(?P<s_y>-?\d+): closest beacon is at x=(?P<b_x>-?\d+), y=(?P<b_y>-?\d+)").unwrap();
        let mut sensors: Vec<Sensor> = Vec::new();
        let mut beacons: HashSet<Sensor> = HashSet::new();

        for caps in digit_finder.captures_iter(input) {
            let s_x = caps.name("s_x").unwrap().as_str().parse::<i32>().unwrap();
            let s_y = caps.name("s_y").unwrap().as_str().parse::<i32>().unwrap();
            let b_x = caps.name("b_x").unwrap().as_str().parse::<i32>().unwrap();
            let b_y = caps.name("b_y").unwrap().as_str().parse::<i32>().unwrap();

            sensors.push(Sensor::new((s_x, s_y), SensorType::Sensor, (b_x, b_y)));
            beacons.insert(Sensor::new((b_x, b_y), SensorType::Beacon, (b_x, b_y)));
        }

        Some((sensors, beacons.into_iter().collect(), false))
    }

    fn silver(&self, (sensors, beacons, test_flag): &Self::Input) -> Option<Self::Output> {
        let max_x = sensors.iter().map(|sensor| sensor.max_x()).max().unwrap();
        let min_x = sensors.iter().map(|sensor| sensor.min_x()).min().unwrap();
        let mut count = 0;

        let y = if *test_flag { 10 } else { 2_000_000 };

        for i in min_x..=max_x {
            if sensors.iter().any(|sensor| sensor.can_be_beacon((i, y)))
                && !beacons
                    .iter()
                    .any(|beacon| beacon.location.0 == i && beacon.location.1 == y)
            {
                count += 1;
            }
        }
        Some((count).into())
    }

    fn gold(&self, (sensors, _, test_flag): &Self::Input) -> Option<Self::Output> {
        let max = if *test_flag { 20 } else { 4_000_000 };
        let mut x: i128 = 0;
        let mut y: i128 = 0;

        for i in 0..=max {
            let line_segments = sensors
                .iter()
                .flat_map(|sensor| sensor.get_segment(i, max))
                .collect::<VecDeque<(i32, i32)>>();

            let resulting_segments = merge_line_segments(line_segments);

            if resulting_segments.len() > 1 {
                x = i as i128;
                y = (std::cmp::max(resulting_segments[0].0, resulting_segments[1].0) - 1) as i128;
                break;
            }
        }

        Some((x * 4_000_000 + y).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let (sensors, beacons, _) = input.unwrap();
        let result = sol.silver(&(sensors, beacons, true)).unwrap();
        assert_eq!(result, 26)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let (sensors, beacons, _) = input.unwrap();
        let result = sol.gold(&(sensors, beacons, true)).unwrap();
        assert_eq!(result, 56_000_011)
    }
}

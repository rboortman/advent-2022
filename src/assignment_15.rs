use std::collections::HashSet;

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

    fn coverage(&self, max_x: i32, max_y: i32) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        for i in std::cmp::max(0, self.location.0 - self.distance_to_beacon)
            ..std::cmp::min(max_x, self.location.0 + self.distance_to_beacon)
        {
            for j in std::cmp::max(0, self.location.1 - self.distance_to_beacon)
                ..std::cmp::min(max_y, self.location.1 + self.distance_to_beacon)
            {
                if self.distance((i, j)) <= self.distance_to_beacon {
                    result.push((i as usize, j as usize));
                }
            }
        }

        result
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
        let max = if *test_flag { 20 } else { 4_000 };
        let mut x = 0;
        let mut y = 0;

        println!("{:?}", sensors);

        let coverage = sensors
            .iter()
            .flat_map(|sensor| sensor.coverage(max, max))
            .collect::<Vec<(usize, usize)>>();

        let mut possible_locations = vec![vec![true; max as usize]; max as usize];

        println!("possible locations len: {}", possible_locations.len());

        for (x, y) in coverage {
            possible_locations[x][y] = false;
        }

        println!("got here");

        for (i, row) in possible_locations.iter().enumerate() {
            if i % 1000 == 0 {
                println!("Row: {}", i);
            }
            match row.iter().position(|b| *b) {
                Some(j) => {
                    x = i as i32;
                    y = j as i32;
                    break;
                }
                None => continue,
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
        assert_eq!(result, 56_000_012)
    }
}

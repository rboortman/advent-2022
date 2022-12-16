use std::collections::{HashMap, HashSet};

use crate::{Assignment, Output};

#[derive(Debug, Clone)]
pub struct Room {
    name: String,
    connected_rooms: Vec<String>,
    flow_rate: i32,
}

impl Room {}

impl std::str::FromStr for Room {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let room_finder = regex::Regex::new(r"\b[A-Z]{2}\b").unwrap();
        let flow_finder = regex::Regex::new(r"\d+").unwrap();

        let mut room_iter = room_finder.captures_iter(s);
        let name = room_iter
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_string();

        let connected_rooms = room_iter
            .map(|cap| cap.get(0).unwrap().as_str().to_string())
            .collect::<Vec<String>>();

        let flow_rate = flow_finder
            .captures(s)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        Ok(Room {
            name,
            connected_rooms,
            flow_rate,
        })
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = HashMap<String, Room>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        let mut map = HashMap::new();
        let rooms = input
            .lines()
            .into_iter()
            .map(|s| s.parse::<Room>().unwrap())
            .collect::<Vec<Room>>();

        for room in rooms {
            map.insert(room.name.clone(), room);
        }

        Some(map)
    }

    fn silver(&self, rooms: &Self::Input) -> Option<Self::Output> {
        let mut paths = HashSet::new();
        paths.insert((String::from("AA"), Vec::new(), 0, 0));

        let max_flow_rate = rooms
            .iter()
            .max_by(|a, b| a.1.flow_rate.cmp(&b.1.flow_rate))
            .unwrap()
            .1
            .flow_rate;

        for _ in 0..30 {
            // println!("Minute: {}\nPaths: {:?}\n", i, paths);
            let mut new_paths = paths
                .iter()
                .flat_map(|(room_id, opened, total_flow, released_pressure)| {
                    let mut branched_paths = HashSet::new();
                    let room = rooms.get(room_id).unwrap();
                    if !opened.contains(room_id) && room.flow_rate > 0 {
                        let mut cloned_opened = opened.clone();
                        cloned_opened.push(room_id.clone());
                        branched_paths.insert((
                            room.name.clone(),
                            cloned_opened,
                            total_flow + room.flow_rate,
                            released_pressure + total_flow,
                        ));
                    }
                    for new_room_id in room.connected_rooms.iter() {
                        branched_paths.insert((
                            new_room_id.clone(),
                            opened.clone(),
                            *total_flow,
                            released_pressure + total_flow,
                        ));
                    }
                    branched_paths
                })
                // .filter(|path| i < 5 || path.2 > 0)
                .collect::<HashSet<(String, Vec<String>, i32, i32)>>();

            let max_released = new_paths.iter().max_by(|a, b| a.3.cmp(&b.3)).unwrap().3;

            new_paths = new_paths
                .into_iter()
                .filter(|(_, _, _, flow_rate)| *flow_rate > (max_released - max_flow_rate))
                .collect::<HashSet<(String, Vec<String>, i32, i32)>>();

            paths = new_paths;
        }

        let max_released = paths.iter().max_by(|a, b| a.3.cmp(&b.3)).unwrap().3;

        Some(max_released.into())
    }

    fn gold(&self, rooms: &Self::Input) -> Option<Self::Output> {
        let mut paths = HashSet::new();
        paths.insert(((String::from("AA"), String::from("AA")), Vec::new(), 0, 0));

        let max_flow_rate = rooms
            .iter()
            .max_by(|a, b| a.1.flow_rate.cmp(&b.1.flow_rate))
            .unwrap()
            .1
            .flow_rate;

        for _ in 0..26 {
            // println!("Minute: {}\nPaths: {:?}\n", i, paths);
            let mut new_paths = paths
                .iter()
                .flat_map(
                    |((room_id_me, room_id_elephant), opened, total_flow, released_pressure)| {
                        let mut branched_paths = HashSet::new();
                        let room_me = rooms.get(room_id_me).unwrap();
                        let room_elephant = rooms.get(room_id_elephant).unwrap();

                        let mut new_rooms_me = room_me.connected_rooms.clone();
                        let mut new_rooms_elephant = room_elephant.connected_rooms.clone();

                        if !opened.contains(room_id_me) && room_me.flow_rate > 0 {
                            new_rooms_me.push(room_id_me.to_owned());
                        }
                        if !opened.contains(room_id_elephant) && room_elephant.flow_rate > 0 {
                            new_rooms_elephant.push(room_id_elephant.to_owned());
                        }

                        for new_room_id_me in new_rooms_me.iter() {
                            for new_room_id_elephant in new_rooms_elephant.iter() {
                                let mut cloned_opened = opened.clone();
                                let mut new_flow_rate = *total_flow;

                                if new_room_id_me.clone() == *room_id_me {
                                    cloned_opened.push(new_room_id_me.clone());
                                    new_flow_rate += room_me.flow_rate;
                                }
                                if new_room_id_elephant.clone() == *room_id_elephant
                                    && new_room_id_elephant.clone() != new_room_id_me.clone()
                                {
                                    cloned_opened.push(new_room_id_elephant.clone());
                                    new_flow_rate += room_elephant.flow_rate;
                                }

                                branched_paths.insert((
                                    (new_room_id_me.clone(), new_room_id_elephant.clone()),
                                    cloned_opened,
                                    new_flow_rate,
                                    released_pressure + total_flow,
                                ));
                            }
                        }
                        branched_paths
                    },
                )
                // .filter(|path| i < 5 || path.2 > 0)
                .collect::<HashSet<((String, String), Vec<String>, i32, i32)>>();

            let max_released = new_paths.iter().max_by(|a, b| a.3.cmp(&b.3)).unwrap().3;

            new_paths = new_paths
                .into_iter()
                .filter(|(_, _, _, flow_rate)| *flow_rate > (max_released - max_flow_rate * 2))
                .collect::<HashSet<((String, String), Vec<String>, i32, i32)>>();

            paths = new_paths;
        }

        let max_released = paths.iter().max_by(|a, b| a.3.cmp(&b.3)).unwrap().3;

        Some(max_released.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 1651)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 1707)
    }
}

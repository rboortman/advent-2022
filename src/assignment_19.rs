use crate::{Assignment, Output};

#[derive(Debug, Clone, Copy)]
enum Robot {
    Ore(i32),
    Clay(i32),
    Obsidian(i32, i32),
    Geode(i32, i32),
}

#[derive(Debug, Clone)]
pub struct Blueprint {
    id: i32,
    ore: Robot,
    clay: Robot,
    obsidian: Robot,
    geode: Robot,
}

impl Blueprint {
    fn get_rundown_evaluation_score(&self) -> (i32, i32, i32, i32) {
        let ore_score = if let Robot::Ore(ore) = self.ore {
            ore
        } else {
            0
        };
        let clay_score = if let Robot::Clay(ore) = self.clay {
            ore * ore_score
        } else {
            0
        };
        let obsidian_score = if let Robot::Obsidian(ore, clay) = self.obsidian {
            ore * ore_score + clay * clay_score
        } else {
            0
        };
        let geode_score = if let Robot::Geode(ore, obsidian) = self.geode {
            ore * ore_score + obsidian * obsidian_score
        } else {
            0
        };

        (ore_score, clay_score, obsidian_score, geode_score)
    }
}

impl std::str::FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digit_finder = regex::Regex::new(r"\d+").unwrap();
        let mut caps = digit_finder.captures_iter(s);

        let id = caps
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let ore_ore = caps
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let clay_ore = caps
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let obsidian_ore = caps
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let obsidian_clay = caps
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let geode_ore = caps
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let geode_obsidian = caps
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        Ok(Blueprint {
            id,
            ore: Robot::Ore(ore_ore),
            clay: Robot::Clay(clay_ore),
            obsidian: Robot::Obsidian(obsidian_ore, obsidian_clay),
            geode: Robot::Geode(geode_ore, geode_obsidian),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct GameState {
    materials: (i32, i32, i32, i32),
    robots: (i32, i32, i32, i32),
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            materials: (0, 0, 0, 0),
            robots: (1, 0, 0, 0),
        }
    }
}

impl GameState {
    fn new(materials: (i32, i32, i32, i32), robots: (i32, i32, i32, i32)) -> GameState {
        GameState { materials, robots }
    }

    fn advance_time(&self, blueprint: &Blueprint) -> Vec<GameState> {
        let mut new_states = Vec::new();
        let new_materials = (
            self.materials.0 + self.robots.0,
            self.materials.1 + self.robots.1,
            self.materials.2 + self.robots.2,
            self.materials.3 + self.robots.3,
        );

        new_states.push(GameState::new(new_materials, self.robots));

        if let Robot::Ore(resource) = blueprint.ore {
            if self.materials.0 >= resource {
                new_states.push(GameState::new(
                    (
                        new_materials.0 - resource,
                        new_materials.1,
                        new_materials.2,
                        new_materials.3,
                    ),
                    (
                        self.robots.0 + 1,
                        self.robots.1,
                        self.robots.2,
                        self.robots.3,
                    ),
                ));
            }
        }
        if let Robot::Clay(resource) = blueprint.clay {
            if self.materials.0 >= resource {
                new_states.push(GameState::new(
                    (
                        new_materials.0 - resource,
                        new_materials.1,
                        new_materials.2,
                        new_materials.3,
                    ),
                    (
                        self.robots.0,
                        self.robots.1 + 1,
                        self.robots.2,
                        self.robots.3,
                    ),
                ));
            }
        }
        if let Robot::Obsidian(resource_ore, resource_clay) = blueprint.obsidian {
            if self.materials.0 >= resource_ore && self.materials.1 >= resource_clay {
                new_states.push(GameState::new(
                    (
                        new_materials.0 - resource_ore,
                        new_materials.1 - resource_clay,
                        new_materials.2,
                        new_materials.3,
                    ),
                    (
                        self.robots.0,
                        self.robots.1,
                        self.robots.2 + 1,
                        self.robots.3,
                    ),
                ));
            }
        }
        if let Robot::Geode(resource_ore, resource_obsidian) = blueprint.geode {
            if self.materials.0 >= resource_ore && self.materials.2 >= resource_obsidian {
                new_states.push(GameState::new(
                    (
                        new_materials.0 - resource_ore,
                        new_materials.1,
                        new_materials.2 - resource_obsidian,
                        new_materials.3,
                    ),
                    (
                        self.robots.0,
                        self.robots.1,
                        self.robots.2,
                        self.robots.3 + 1,
                    ),
                ));
            }
        }
        new_states
    }

    fn get_geodes(&self) -> i32 {
        self.materials.3
    }

    fn get_evaluation_score(&self, blueprint: &Blueprint) -> i32 {
        let (ore_score, clay_score, obsidian_score, geode_score) =
            blueprint.get_rundown_evaluation_score();

        (self.materials.0 + self.robots.0) * ore_score
            + (self.materials.1 + self.robots.1) * clay_score
            + (self.materials.2 + self.robots.2) * obsidian_score
            + (self.materials.3 + self.robots.3) * geode_score
    }
}

fn check_blueprint(blueprint: &Blueprint, minutes: i32) -> i32 {
    let mut states = vec![(GameState::default(), Vec::new())];
    let score_cap = 5000;

    for _i in 0..minutes {
        let mut new_states = Vec::new();
        for (state, mut previous_state) in states {
            previous_state.push(state);
            let mut advanced_states = state
                .advance_time(blueprint)
                .into_iter()
                .map(|s| (s, previous_state.clone()))
                .collect();
            new_states.append(&mut advanced_states);
        }
        new_states.sort_by_cached_key(|(state, _)| state.get_evaluation_score(blueprint));
        states = new_states.into_iter().rev().take(score_cap).collect();
    }

    states
        .iter()
        .map(|(state, _)| state)
        .max_by_key(|state| state.get_geodes())
        .unwrap()
        .get_geodes()
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Vec<Blueprint>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        Some(input.lines().map(|s| s.parse().unwrap()).collect())
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        let game_rounds = 24;
        Some(
            input
                .iter()
                .map(|blueprint| check_blueprint(blueprint, game_rounds) * blueprint.id)
                .sum::<i32>()
                .into(),
        )
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        let game_rounds = 32;
        Some(
            input
                .iter()
                .take(3)
                .map(|blueprint| check_blueprint(blueprint, game_rounds))
                .product::<i32>()
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 33)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 3472)
    }
}

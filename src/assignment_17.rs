use crate::{Assignment, Output};

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Shape {
    HLine,
    Plus,
    L,
    VLine,
    Block,
}

static FALL_ORDER: [Shape; 5] = [
    Shape::HLine,
    Shape::Plus,
    Shape::L,
    Shape::VLine,
    Shape::Block,
];

impl std::convert::From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Couldn't parse {}", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FieldType {
    Empty,
    SteadyRock(Shape),
    FallingRock(Shape),
}

#[derive(Debug)]
struct PlayField {
    field: Vec<Vec<FieldType>>,
}

impl PlayField {
    fn new() -> PlayField {
        PlayField {
            field: vec![vec![FieldType::Empty; 7]; 1],
        }
    }

    fn heighest_rock(&self) -> usize {
        let mut i = 0;
        for row in self.field.iter() {
            if row.iter().any(|cell| *cell != FieldType::Empty) {
                i += 1;
            } else {
                break;
            }
        }
        i
    }

    fn enlarge_field(&self, to_size: usize) -> PlayField {
        let mut field_clone = self.field.clone();
        if to_size <= field_clone.len() {
            return PlayField { field: field_clone };
        }
        let mut to_add = vec![vec![FieldType::Empty; 7]; to_size - field_clone.len()];
        field_clone.append(&mut to_add);
        PlayField { field: field_clone }
    }

    fn add_rocks(&self, rock: &Shape) -> PlayField {
        let heighest_rock = self.heighest_rock();
        let mut new_self = self.enlarge_field(heighest_rock + 7);

        match rock {
            Shape::HLine => {
                new_self.field[heighest_rock + 3][2] = FieldType::FallingRock(Shape::HLine);
                new_self.field[heighest_rock + 3][3] = FieldType::FallingRock(Shape::HLine);
                new_self.field[heighest_rock + 3][4] = FieldType::FallingRock(Shape::HLine);
                new_self.field[heighest_rock + 3][5] = FieldType::FallingRock(Shape::HLine);
            }
            Shape::Plus => {
                new_self.field[heighest_rock + 3][3] = FieldType::FallingRock(Shape::Plus);
                new_self.field[heighest_rock + 4][2] = FieldType::FallingRock(Shape::Plus);
                new_self.field[heighest_rock + 4][3] = FieldType::FallingRock(Shape::Plus);
                new_self.field[heighest_rock + 4][4] = FieldType::FallingRock(Shape::Plus);
                new_self.field[heighest_rock + 5][3] = FieldType::FallingRock(Shape::Plus);
            }
            Shape::L => {
                new_self.field[heighest_rock + 3][2] = FieldType::FallingRock(Shape::L);
                new_self.field[heighest_rock + 3][3] = FieldType::FallingRock(Shape::L);
                new_self.field[heighest_rock + 3][4] = FieldType::FallingRock(Shape::L);
                new_self.field[heighest_rock + 4][4] = FieldType::FallingRock(Shape::L);
                new_self.field[heighest_rock + 5][4] = FieldType::FallingRock(Shape::L);
            }
            Shape::VLine => {
                new_self.field[heighest_rock + 3][2] = FieldType::FallingRock(Shape::VLine);
                new_self.field[heighest_rock + 4][2] = FieldType::FallingRock(Shape::VLine);
                new_self.field[heighest_rock + 5][2] = FieldType::FallingRock(Shape::VLine);
                new_self.field[heighest_rock + 6][2] = FieldType::FallingRock(Shape::VLine);
            }
            Shape::Block => {
                new_self.field[heighest_rock + 3][2] = FieldType::FallingRock(Shape::Block);
                new_self.field[heighest_rock + 3][3] = FieldType::FallingRock(Shape::Block);
                new_self.field[heighest_rock + 4][2] = FieldType::FallingRock(Shape::Block);
                new_self.field[heighest_rock + 4][3] = FieldType::FallingRock(Shape::Block);
            }
        }
        new_self
    }

    fn has_movable_rocks(&self) -> bool {
        self.field.iter().any(|row| {
            row.iter().any(|cell| {
                FALL_ORDER
                    .iter()
                    .any(|shape| *cell == FieldType::FallingRock(*shape))
            })
        })
    }

    fn can_rock_move(&self, direction: &Direction) -> bool {
        self.field.iter().enumerate().all(|(i, row)| {
            row.iter().enumerate().all(|(j, cell)| {
                if let FieldType::FallingRock(shape) = *cell {
                    match direction {
                        Direction::Left => {
                            j > 0
                                && (row[j - 1] == FieldType::Empty
                                    || row[j - 1] == FieldType::FallingRock(shape))
                        }
                        Direction::Right => {
                            j < row.len() - 1
                                && (row[j + 1] == FieldType::Empty
                                    || row[j + 1] == FieldType::FallingRock(shape))
                        }
                        Direction::Down => {
                            i > 0
                                && (self.field[i - 1][j] == FieldType::Empty
                                    || self.field[i - 1][j] == FieldType::FallingRock(shape))
                        }
                    }
                } else {
                    true
                }
            })
        })
    }

    fn move_rock(&self, direction: &Direction) -> PlayField {
        let mut field_clone = self.field.clone();

        if !self.can_rock_move(direction) {
            if *direction == Direction::Down {
                field_clone = field_clone
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|cell| match cell {
                                FieldType::Empty => FieldType::Empty,
                                FieldType::SteadyRock(shape) => FieldType::SteadyRock(*shape),
                                FieldType::FallingRock(shape) => FieldType::SteadyRock(*shape),
                            })
                            .collect::<Vec<FieldType>>()
                    })
                    .collect::<Vec<Vec<FieldType>>>();
            }

            return PlayField { field: field_clone };
        }

        for (i, row) in field_clone.clone().iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match direction {
                    Direction::Left => {
                        if let FieldType::FallingRock(shape) = *cell {
                            field_clone[i][j] = FieldType::Empty;
                            field_clone[i][j - 1] = FieldType::FallingRock(shape);
                        }
                    }
                    Direction::Right => {
                        let j = row.len() - j - 1;
                        let cell = &field_clone[i][j];
                        if let FieldType::FallingRock(shape) = *cell {
                            field_clone[i][j] = FieldType::Empty;
                            field_clone[i][j + 1] = FieldType::FallingRock(shape);
                        }
                    }
                    Direction::Down => {
                        if let FieldType::FallingRock(shape) = *cell {
                            field_clone[i][j] = FieldType::Empty;
                            field_clone[i - 1][j] = FieldType::FallingRock(shape);
                        }
                    }
                }
            }
        }

        PlayField { field: field_clone }
    }

    fn look_for_pattern(&self) -> Option<(usize, usize)> {
        let heighest = self.heighest_rock() - 1;
        let mut has_pattern = false;
        let mut matched = 0;
        let window_checker = 30;

        for i in window_checker..heighest {
            let mut loop_check = true;

            for j in 0..window_checker {
                loop_check = loop_check && self.field[i - j] == self.field[heighest - j];
            }
            has_pattern = has_pattern || loop_check;

            if has_pattern {
                matched = i;
                break;
            }
        }

        if has_pattern {
            let mut total = 0;
            for i in matched..heighest {
                total += self.field[i]
                    .iter()
                    .filter(|cell| **cell != FieldType::Empty)
                    .count();
            }
            Some((heighest - matched, (total / 22) * 5))
        } else {
            None
        }
    }
}

impl std::fmt::Display for PlayField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut to_write = self
            .field
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        FieldType::Empty => ".",
                        FieldType::SteadyRock(shape) => match shape {
                            // "█"
                            Shape::HLine => "A",
                            Shape::Plus => "B",
                            Shape::L => "C",
                            Shape::VLine => "D",
                            Shape::Block => "E",
                        },
                        FieldType::FallingRock(_) => "#",
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>();
        to_write.reverse();

        write!(f, "{}", to_write.join("\n"))
    }
}

fn play_tetris(input: &Vec<Direction>, output_size: i64) -> Option<Output> {
    let mut field = PlayField::new();
    let mut rocks_fallen = 0;
    let mut direction_index = 0;
    let mut gas_index = 0;
    let input_len = input.len();
    field = field.add_rocks(&FALL_ORDER[0]);

    let mut add_to_result = 0;

    while rocks_fallen < output_size {
        if !field.has_movable_rocks() {
            rocks_fallen += 1;
            direction_index = 0;
            field = field.add_rocks(&FALL_ORDER[(rocks_fallen % 5) as usize]);

            if add_to_result == 0 {
                match field.look_for_pattern() {
                    None => continue,
                    Some((height, blocks)) => {
                        let jump_blocks = (output_size - rocks_fallen) / (blocks as i64);
                        rocks_fallen += jump_blocks * (blocks as i64);
                        add_to_result = jump_blocks * (height as i64);
                    }
                }
            }
        } else {
            let direction = if direction_index % 2 != 0 {
                &Direction::Down
            } else {
                &input[gas_index % input_len]
            };

            field = field.move_rock(direction);

            if direction_index % 2 == 0 {
                gas_index += 1;
            }
            direction_index += 1;
        }
    }

    // println!("{}", field);

    Some((add_to_result + field.heighest_rock() as i64).into())
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Vec<Direction>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        Some(input.trim().chars().map(Direction::from).collect())
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        play_tetris(input, 2022)

        // let mut field = PlayField::new();
        // let mut rocks_fallen = 0;
        // let mut direction_index = 0;
        // let mut gas_index = 0;
        // let input_len = input.len();
        // field = field.add_rocks(&FALL_ORDER[0]);

        // while rocks_fallen < 10 {
        //     if !field.has_movable_rocks() {
        //         rocks_fallen += 1;
        //         direction_index = 0;
        //         field = field.add_rocks(&FALL_ORDER[rocks_fallen % 5]);
        //     } else {
        //         let direction = if direction_index % 2 != 0 {
        //             &Direction::Down
        //         } else {
        //             &input[gas_index % input_len]
        //         };

        //         field = field.move_rock(direction);

        //         if direction_index % 2 == 0 {
        //             gas_index += 1;
        //         }
        //         direction_index += 1;
        //     }
        // }

        // // println!("{}", field);

        // Some((field.heighest_rock() as i64).into())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        play_tetris(input, 1_000_000_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 3068)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.gold(&input.unwrap()).unwrap();
        let should_be: i64 = 1_514_285_714_288;
        assert_eq!(result, should_be)
    }
}

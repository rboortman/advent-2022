use crate::{Assignment, Output};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    None,
    Empty,
    Wall,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            ' ' => Tile::None,
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            _ => Tile::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
    player_coords: (usize, usize),
    facing: Direction,
    cube: Option<Vec<Vec<Vec<Vec<Tile>>>>>,
    cube_player_coords: (usize, usize, usize, usize),
}

impl Board {
    fn get_movement_vector(&self) -> (i32, i32) {
        match self.facing {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
        }
    }

    fn move_player(&mut self) {
        if self.cube.is_none() {
            let move_vector = self.get_movement_vector();

            let v_length = self.tiles.len();
            let h_length = self.tiles[self.player_coords.0].len();

            let len = match self.facing {
                Direction::Right | Direction::Left => h_length,
                Direction::Down | Direction::Up => v_length,
            };

            for i in 1..len {
                let mut new_y =
                    (self.player_coords.0 as i32 + (i as i32 * move_vector.0)) % v_length as i32;
                let mut new_x =
                    (self.player_coords.1 as i32 + (i as i32 * move_vector.1)) % h_length as i32;

                if new_y.is_negative() {
                    new_y += v_length as i32;
                }
                if new_x.is_negative() {
                    new_x += h_length as i32;
                }

                let new_coord = (new_y, new_x);

                match self.tiles[new_coord.0 as usize].get(new_coord.1 as usize) {
                    None => continue,
                    Some(tile) => match tile {
                        Tile::None => continue,
                        Tile::Wall => break,
                        Tile::Empty => {
                            self.player_coords = (new_coord.0 as usize, new_coord.1 as usize);
                            break;
                        }
                    },
                }
            }
        } else {
            let cube = self.cube.clone().unwrap();
            let len = cube[0][0].len();

            let mut new_cube_y = self.cube_player_coords.0;
            let mut new_cube_x = self.cube_player_coords.1;
            let mut new_y = self.cube_player_coords.2;
            let mut new_x = self.cube_player_coords.3;
            let mut new_facing = self.facing.clone();

            match self.facing {
                Direction::Right => {
                    if new_x < len - 1 {
                        new_x += 1;
                    } else {
                        match (new_cube_y, new_cube_x) {
                            (0, _) => {
                                new_cube_y = 1;
                                new_cube_x = 1;
                                new_y = 0;
                                new_x = len - 1 - self.cube_player_coords.2;
                                new_facing = Direction::Down;
                            }
                            (1, _) => {
                                new_cube_x = (new_cube_x + 1) % 4;
                                new_x = 0;
                            }
                            (2, _) => {
                                new_cube_y = 1;
                                new_cube_x = 1;
                                new_y = len - 1;
                                new_x = self.cube_player_coords.2;
                                new_facing = Direction::Up;
                            }
                            _ => panic!("something went wrong when moving"),
                        }
                    }
                }
                Direction::Left => {
                    if new_x > 0 {
                        new_x -= 1;
                    } else {
                        match (new_cube_y, new_cube_x) {
                            (0, _) => {
                                new_cube_y = 1;
                                new_cube_x = 3;
                                new_y = 0;
                                new_x = self.cube_player_coords.2;
                                new_facing = Direction::Down;
                            }
                            (1, _) => {
                                new_cube_x = (4 + new_cube_x - 1) % 4;
                                new_x = len - 1;
                            }
                            (2, _) => {
                                new_cube_y = 1;
                                new_cube_x = 1;
                                new_y = len - 1;
                                new_x = len - 1 - self.cube_player_coords.2;
                                new_facing = Direction::Up;
                            }
                            _ => panic!("something went wrong when moving"),
                        }
                    }
                }
                Direction::Down => {
                    if new_y < len - 1 {
                        new_y += 1;
                    } else {
                        match (new_cube_y, new_cube_x) {
                            (0, 0) | (1, 0) => {
                                new_cube_y += 1;
                                new_y = 0;
                            }
                            (1, 1) => {
                                new_cube_y += 1;
                                new_cube_x = 0;
                                new_y = self.cube_player_coords.3;
                                new_x = len - 1;
                                new_facing = Direction::Left
                            }
                            (1, 2) => {
                                new_cube_y += 1;
                                new_cube_x = 0;
                                new_y = len - 1;
                                new_x = len - 1 - self.cube_player_coords.3;
                                new_facing = Direction::Up
                            }
                            (1, 3) => {
                                new_cube_y += 1;
                                new_cube_x = 0;
                                new_y = len - 1 - self.cube_player_coords.3;
                                new_x = 0;
                                new_facing = Direction::Right
                            }
                            (2, 0) => {
                                new_cube_y = 1;
                                new_cube_x = 2;
                                new_y = len - 1;
                                new_x = len - 1 - self.cube_player_coords.3;
                                new_facing = Direction::Up
                            }
                            _ => panic!("something went wrong when moving"),
                        }
                    }
                }
                Direction::Up => {
                    if new_y > 0 {
                        new_y -= 1;
                    } else {
                        match (new_cube_y, new_cube_x) {
                            (0, 0) => {
                                new_cube_y += 1;
                                new_cube_x = 2;
                                new_y = 0;
                                new_x = len - 1 - self.cube_player_coords.3;
                                new_facing = Direction::Down
                            }
                            (1, 1) => {
                                new_cube_y -= 1;
                                new_cube_x = 0;
                                new_y = len - 1 - self.cube_player_coords.3;
                                new_x = len - 1;
                                new_facing = Direction::Left
                            }
                            (1, 2) => {
                                new_cube_y -= 1;
                                new_cube_x = 0;
                                new_y = 0;
                                new_x = len - 1 - self.cube_player_coords.3;
                                new_facing = Direction::Down
                            }
                            (1, 3) => {
                                new_cube_y -= 1;
                                new_cube_x = 0;
                                new_y = self.cube_player_coords.3;
                                new_x = 0;
                                new_facing = Direction::Right
                            }
                            (1, 0) | (2, 0) => {
                                new_cube_y -= 1;
                                new_y = len - 1;
                            }
                            _ => panic!("something went wrong when moving"),
                        }
                    }
                }
            }

            match cube[new_cube_y][new_cube_x][new_y][new_x] {
                Tile::None => panic!("Encountered a None in a cube"),
                Tile::Wall => {}
                Tile::Empty => {
                    self.cube_player_coords = (new_cube_y, new_cube_x, new_y, new_x);
                    self.facing = new_facing;
                }
            }
        }
    }

    fn process_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(to_move) => {
                for _ in 0..*to_move {
                    self.move_player();
                }
            }
            _ => {
                self.facing = self.facing.rotate(instruction);
            }
        }
    }

    fn get_score(&self) -> i32 {
        (1 + self.player_coords.0 as i32) * 1000
            + (1 + self.player_coords.1 as i32) * 4
            + self.facing.get_score()
    }

    fn convert_to_cube(&mut self, plane_size: usize) {
        let mut cube = vec![vec![vec![vec![Tile::None; plane_size]; plane_size]; 4]; 4];

        for (i, row) in self.tiles.iter().enumerate() {
            let i_cube = i / plane_size;
            for (j, tile) in row.iter().enumerate() {
                let j_cube = j / plane_size;
                cube[i_cube][j_cube][i % plane_size][j % plane_size] = tile.clone();
            }
        }

        let cube_coords = self
            .tiles
            .chunks(plane_size)
            .enumerate()
            .flat_map(|(i, rows)| {
                rows[0]
                    .chunks(plane_size)
                    .enumerate()
                    .map(|(j, tiles)| {
                        if tiles[0] == Tile::None {
                            (i, j, false)
                        } else {
                            (i, j, true)
                        }
                    })
                    .filter(|(_, _, bool)| *bool)
                    .map(|(i, j, _)| (i, j))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();

        let top_coords = cube_coords[0];
        let top = &cube[top_coords.0][top_coords.1];
        let mut center = cube[top_coords.0 + 1].clone();
        let bottom = &cube[top_coords.0 + 2][top_coords.1];

        let mut added_to_left = 0;

        for (y, x) in cube_coords {
            if x == top_coords.1 || y == top_coords.0 + 1 {
                continue;
            }

            match (y < top_coords.0 + 1, x < top_coords.1) {
                (true, true) => {
                    let to_add = translate_plane(
                        &cube[y][x],
                        TranslateDirection::CounterClockwise,
                        y.abs_diff(top_coords.0 + 1),
                    );
                    center.splice(0..1, Vec::from([to_add]));
                    added_to_left += 1;
                }
                (true, false) => center.push(translate_plane(
                    &cube[y][x],
                    TranslateDirection::Clockwise,
                    y.abs_diff(top_coords.0 + 1),
                )),
                (false, true) => {
                    let to_add = translate_plane(
                        &cube[y][x],
                        TranslateDirection::Clockwise,
                        y.abs_diff(top_coords.0 + 1),
                    );
                    center.splice(0..1, Vec::from([to_add]));
                    added_to_left += 1;
                }
                (false, false) => center.push(translate_plane(
                    &cube[y][x],
                    TranslateDirection::CounterClockwise,
                    y.abs_diff(top_coords.0 + 1),
                )),
            }
        }

        for _ in 0..(added_to_left + top_coords.1) {
            let to_add = center.remove(0);
            center.push(to_add);
        }

        center = center
            .into_iter()
            .filter(|plane| plane[0][0] != Tile::None)
            .collect::<Vec<Vec<Vec<Tile>>>>();

        self.cube = Some(vec![vec![top.clone()], center, vec![bottom.clone()]]);
    }
}

impl std::str::FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|row| row.chars().map(Tile::from).collect::<Vec<Tile>>())
            .collect::<Vec<Vec<Tile>>>();

        let mut player_coords = (0, 0);

        'outer: for (i, row) in tiles.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if *tile == Tile::Empty {
                    player_coords = (i, j);
                    break 'outer;
                }
            }
        }

        Ok(Board {
            tiles,
            player_coords,
            facing: Direction::Right,
            cube: None,
            cube_player_coords: (0, 0, 0, 0),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn rotate(&self, instruction: &Instruction) -> Direction {
        match (self, instruction) {
            (Direction::Right, Instruction::RotateClockwise) => Direction::Down,
            (Direction::Right, Instruction::RotateCounterClockwise) => Direction::Up,
            (Direction::Down, Instruction::RotateClockwise) => Direction::Left,
            (Direction::Down, Instruction::RotateCounterClockwise) => Direction::Right,
            (Direction::Left, Instruction::RotateClockwise) => Direction::Up,
            (Direction::Left, Instruction::RotateCounterClockwise) => Direction::Down,
            (Direction::Up, Instruction::RotateClockwise) => Direction::Right,
            (Direction::Up, Instruction::RotateCounterClockwise) => Direction::Left,
            (_, Instruction::Move(_)) => self.clone(),
        }
    }

    fn get_score(&self) -> i32 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Move(i32),
    RotateClockwise,
    RotateCounterClockwise,
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    use pom::char_class::*;
    use pom::parser::*;

    let turn = sym(b'L').map(|_| Instruction::RotateCounterClockwise)
        | sym(b'R').map(|_| Instruction::RotateClockwise);
    let move_size = || {
        is_a(digit).repeat(1..).map(|digits| {
            Instruction::Move(
                digits
                    .iter()
                    .fold(0i32, |result, digit| result * 10 + (digit - b'0') as i32),
            )
        })
    };

    let parser =
        (move_size() + (turn + move_size()).repeat(1..)).map(|(first_step, turns_and_steps)| {
            let mut instructions = vec![first_step];
            instructions.append(
                &mut turns_and_steps
                    .into_iter()
                    .flat_map(|(i1, i2)| vec![i1, i2])
                    .collect::<Vec<Instruction>>(),
            );

            instructions
        });

    parser
        .parse(s.as_bytes())
        .map_err(|e| e.to_string())
        .unwrap()
}

enum TranslateDirection {
    Clockwise,
    CounterClockwise,
}

fn translate_plane(
    plane: &Vec<Vec<Tile>>,
    direction: TranslateDirection,
    times: usize,
) -> Vec<Vec<Tile>> {
    let mut new_plane = plane.clone();

    for _ in 0..times {
        let mut temp_plane = new_plane.clone();
        for (i, row) in new_plane.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                match direction {
                    TranslateDirection::Clockwise => {
                        temp_plane[j][new_plane.len() - 1 - i] = tile.clone()
                    }
                    TranslateDirection::CounterClockwise => {
                        temp_plane[new_plane.len() - 1 - j][i] = tile.clone()
                    }
                }
            }
        }
        new_plane = temp_plane
    }

    new_plane
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = (Board, Vec<Instruction>, bool);
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        let (board_str, instructions_str) = input.split_once("\n\n").unwrap();
        Some((
            board_str.parse::<Board>().unwrap(),
            parse_instructions(instructions_str),
            false,
        ))
    }

    fn silver(&self, (board, instructions, _): &Self::Input) -> Option<Self::Output> {
        let mut board_clone = board.clone();
        let instructions_clone = instructions.clone();

        for instruction in &instructions_clone {
            board_clone.process_instruction(instruction);
        }

        Some(board_clone.get_score().into())
    }

    fn gold(&self, (board, instructions, test_flag): &Self::Input) -> Option<Self::Output> {
        let plane_size = if *test_flag { 4 } else { 50 };
        let mut board_clone = board.clone();
        let instructions_clone = instructions.clone();

        board_clone.convert_to_cube(plane_size);

        for instruction in &instructions_clone {
            board_clone.process_instruction(instruction);
        }

        println!(
            "({}, {}, {}, {}), Facing: {:?}",
            board.cube_player_coords.0,
            board.cube_player_coords.1,
            board.cube_player_coords.2,
            board.cube_player_coords.3,
            board.facing
        );

        Some((-1).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 6032)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let (board, instructions, _) = input.unwrap();
        let result = sol.gold(&(board, instructions, true)).unwrap();
        assert_eq!(result, 5031)
    }
}

use crate::define_examples;
use crate::*;

static NUMBER_OF_CYCLES: Int = 1000000000;

type Grid<T> = Vec<Vec<T>>;

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

use std::{collections::HashMap, fmt::Debug};

use Direction::*;

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
struct Point {
    x: Int,
    y: Int,
}

impl Point {
    fn move_to(&self, direction: Direction) -> Point {
        match direction {
            North => Point {
                x: self.x,
                y: self.y - 1,
            },
            East => Point {
                x: self.x + 1,
                y: self.y,
            },
            South => Point {
                x: self.x,
                y: self.y + 1,
            },
            West => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn new(x: Int, y: Int) -> Point {
        Point { x, y }
    }
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
enum Terrain {
    RoundedRock(Option<Point>),
    CubeShapedRock(Option<Point>),
    EmptySpace(Option<Point>),
}

impl Terrain {
    fn from_char(c: char) -> Terrain {
        use Terrain::*;
        match c {
            'O' => RoundedRock(None),
            '#' => CubeShapedRock(None),
            '.' => EmptySpace(None),
            _ => panic!("Invalid Terrain char: '{}'.", c),
        }
    }

    fn to_char(&self) -> char {
        use Terrain::*;
        match &self {
            RoundedRock(_) => 'O',
            CubeShapedRock(_) => '#',
            EmptySpace(_) => '.',
        }
    }
}

#[derive(PartialEq, Clone)]
struct Platform {
    rows: usize,
    columns: usize,
    grid: Grid<Terrain>,
}

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Platform({}x{}):\n{}",
            self.rows,
            self.columns,
            grid_to_string(&self.grid)
        )
    }
}

impl Platform {
    fn parse(input: Input) -> Platform {
        Platform::from_strings(input.lines().map(|s| s.to_string()).collect::<Vec<_>>())
    }

    fn run_spin_cycle(&mut self) {
        vec![North, West, South, East]
            .iter()
            .for_each(|d| self.tilt(d.clone()));
    }

    fn tilt(&mut self, direction: Direction) {
        match direction {
            North => {
                for y in 0..self.rows {
                    for x in 0..self.columns {
                        self.tilt_terrain(x, y, &direction);
                    }
                }
            }
            South => {
                for y in (0..self.rows).rev() {
                    for x in 0..self.columns {
                        self.tilt_terrain(x, y, &direction);
                    }
                }
            }
            East => {
                for x in (0..self.columns).rev() {
                    for y in 0..self.rows {
                        self.tilt_terrain(x, y, &direction);
                    }
                }
            }
            West => {
                for x in 0..self.columns {
                    for y in 0..self.rows {
                        self.tilt_terrain(x, y, &direction);
                    }
                }
            }
        }
        self.assign_points();
    }

    fn tilt_terrain(&mut self, x: usize, y: usize, direction: &Direction) {
        let point = Point::new(x as Int, y as Int);
        self.tilt_terrain_at_point(&point, direction);
    }

    fn tilt_terrain_at_point(&mut self, point: &Point, direction: &Direction) {
        let mut point = point.clone();
        while self.can_move(&point, direction.clone()) {
            let to = point.move_to(direction.clone());
            self.move_to(&point, direction.clone());
            point = to;
        }
    }

    fn move_to(&mut self, from_point: &Point, direction: Direction) {
        let from = self.get(from_point).unwrap().clone();
        let to = self.get(&from_point.move_to(direction)).unwrap().clone();
        match (&from, &to) {
            (Terrain::RoundedRock(_), Terrain::EmptySpace(Some(to_point))) => {
                self.set(&to_point, &Terrain::RoundedRock(Some(to_point.clone())));
                self.set(from_point, &Terrain::EmptySpace(Some(from_point.clone())));
            }
            (Terrain::EmptySpace(_) | Terrain::CubeShapedRock(_), _) => (),
            _ => panic!("Not able to move from '{:?}' to '{:?}'.", from, to),
        }
    }

    fn can_move(&self, point: &Point, direction: Direction) -> bool {
        let from = self.get(&point).unwrap();
        let to = self.get(&point.move_to(direction));
        match (from, to) {
            (Terrain::RoundedRock(_), Some(Terrain::EmptySpace(_))) => true,
            _ => false,
        }
    }

    fn point_outside_grid(&self, point: &Point) -> bool {
        point.x < 0 || point.x >= self.columns as Int || point.y < 0 || point.y >= self.rows as Int
    }

    fn get(&self, point: &Point) -> Option<&Terrain> {
        let point_outside_grid = self.point_outside_grid(point);
        if point_outside_grid {
            None
        } else {
            Some(&self.grid[point.y as usize][point.x as usize])
        }
    }

    fn set(&mut self, point: &Point, terrain: &Terrain) {
        assert!(
            !self.point_outside_grid(point),
            "Point not inside grid: '{:?}'.",
            point
        );
        self.grid[point.y as usize][point.x as usize] = terrain.clone();
    }

    fn load_at_row(&self, row: Int) -> Option<Int> {
        if row < 0 || row >= self.rows as Int {
            None
        } else {
            Some(self.rows as Int - row)
        }
    }

    fn get_total_load(&self) -> Int {
        let mut total_load = 0;
        for y in 0..self.rows {
            let load_at_row = self.load_at_row(y as Int).unwrap();
            for x in 0..self.columns {
                let point = Point::new(x as Int, y as Int);
                let terrain = self.get(&point).unwrap();
                match terrain {
                    Terrain::RoundedRock(_) => total_load += load_at_row,
                    _ => (),
                }
            }
        }
        total_load
    }

    fn assign_points(&mut self) {
        for y in 0..self.rows {
            for x in 0..self.columns {
                let point = Point::new(x as Int, y as Int);
                let terrain = self.get(&point).unwrap();
                let new_terrain = match terrain {
                    Terrain::CubeShapedRock(_) => Terrain::CubeShapedRock(Some(point.clone())),
                    Terrain::RoundedRock(_) => Terrain::RoundedRock(Some(point.clone())),
                    Terrain::EmptySpace(_) => Terrain::EmptySpace(Some(point.clone())),
                };
                self.set(&point, &new_terrain);
            }
        }
    }

    fn get_total_load_after_cycles(&mut self, number_of_cycles: Int) -> Int {
        let mut grid_at: HashMap<Grid<Terrain>, Int> = HashMap::new();
        for current_cycle in 1..number_of_cycles {
            self.run_spin_cycle();
            if let Some(previous_grid_at) = grid_at.insert(self.grid.clone(), current_cycle) {
                let cycles_left = number_of_cycles - current_cycle;
                let number_of_cycles_in_loop = current_cycle - previous_grid_at;
                if cycles_left % number_of_cycles_in_loop == 0 {
                    break;
                }
            }
        }
        self.get_total_load()
    }

    fn from_strings(input: Vec<String>) -> Platform {
        let grid: Grid<Terrain> = input
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().map(|c| Terrain::from_char(c)).collect())
            .collect();
        let rows = grid.len();
        let columns = grid.first().unwrap().len();
        let mut result = Platform {
            rows,
            columns,
            grid,
        };
        result.assign_points();
        result
    }
}

fn grid_to_string(grid: &Grid<Terrain>) -> String {
    let mut result: String = String::new();
    let rows = grid.len();
    let columns = grid.first().unwrap().len();
    for y in 0..rows {
        for x in 0..columns {
            let terrain = &grid[y][x];
            result.push(terrain.to_char());
        }
        result.push('\n');
    }
    result
}

#[derive(Default)]
pub struct Problem {}

impl Solution for Problem {
    fn year(&self) -> Year {
        2023
    }
    fn day(&self) -> Day {
        14
    }
    fn expect_part_one(&self) -> Answer {
        109098
    }
    fn expect_part_two(&self) -> Answer {
        100064
    }

    define_examples! {
        (
            "
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
            ",
            Expect::PartsOneAndTwo(136, 64),
        )
    }

    fn solve_part_one(&self, input: Input, _is_example: bool) -> Answer {
        let mut platform = Platform::parse(input);
        platform.tilt(North);
        let total_load = platform.get_total_load();
        total_load
    }

    fn solve_part_two(&self, input: Input, _is_example: bool) -> Answer {
        let total_load_after_many_cycles =
            Platform::parse(input).get_total_load_after_cycles(NUMBER_OF_CYCLES);
        total_load_after_many_cycles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tilt_part_one() {
        let d14 = Problem {};
        let mut example_platform =
            Platform::parse(d14.define_examples().first().unwrap().get_input());
        dbg!(&example_platform);
        example_platform.tilt(North);
        dbg!(&example_platform);
        let example_total_load = example_platform.get_total_load();
        test!(136, example_total_load);
        let example_input_tilted = vec_of_strings![
            "OOOO.#.O..",
            "OO..#....#",
            "OO..O##..O",
            "O..#.OO...",
            "........#.",
            "..#....#.#",
            "..O..#.O.O",
            "..O.......",
            "#....###..",
            "#....#....",
        ];
        let example_platform_tilted = Platform::from_strings(example_input_tilted);
        //dbg!(&example_platform_tilted);
        for y in 0..example_platform_tilted.rows {
            for x in 0..example_platform_tilted.columns {
                let point = Point::new(x as Int, y as Int);
                let tilted_terrain = example_platform_tilted.get(&point).unwrap();
                test!(
                    tilted_terrain,
                    example_platform.get(&point).unwrap(),
                    "Tilt: {:?}",
                    point
                );
            }
        }
    }

    #[test]
    fn test_tilt_part_two() {
        let d14 = Problem {};
        let mut example_platform =
            Platform::parse(d14.define_examples().first().unwrap().get_input());
        let example_platform_1_cycle = Platform::from_strings(vec_of_strings![
            ".....#....",
            "....#...O#",
            "...OO##...",
            ".OO#......",
            ".....OOO#.",
            ".O#...O#.#",
            "....O#....",
            "......OOOO",
            "#...O###..",
            "#..OO#....",
        ]);
        let example_platform_2_cycle = Platform::from_strings(vec_of_strings![
            ".....#....",
            "....#...O#",
            ".....##...",
            "..O#......",
            ".....OOO#.",
            ".O#...O#.#",
            "....O#...O",
            ".......OOO",
            "#..OO###..",
            "#.OOO#...O",
        ]);
        let example_platform_3_cycle = Platform::from_strings(vec_of_strings![
            ".....#....",
            "....#...O#",
            ".....##...",
            "..O#......",
            ".....OOO#.",
            ".O#...O#.#",
            "....O#...O",
            ".......OOO",
            "#...O###.O",
            "#.OOO#...O",
        ]);
        example_platform.run_spin_cycle();
        test!(example_platform_1_cycle, example_platform);
        example_platform.run_spin_cycle();
        test!(example_platform_2_cycle, example_platform);
        example_platform.run_spin_cycle();
        test!(example_platform_3_cycle, example_platform);
    }
}

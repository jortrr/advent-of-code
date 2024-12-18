use crate::*;

use std::{cmp::max, collections::HashSet};

use grid::*;

type Beam = Direction;
type Points = HashSet<Point>;

#[derive(Debug)]
struct Terrain {
    the_type: TerrainType,
    beams: Vec<Beam>,
    energized: bool,
}

impl Terrain {
    fn from_char(c: char) -> Terrain {
        Terrain {
            the_type: TerrainType::from_char(c),
            beams: Vec::new(),
            energized: false,
        }
    }
}

define_convertable_enum! {
    TerrainType {
        EmptySpace => '.',
        NorthEastMirror => '/',
        NorthWestMirror => '\\',
        NorthSouthSplitter => '|',
        EastWestSplitter => '-',
    }
}

impl TerrainType {
    fn redirect(&self, beam: &Beam) -> Vec<Beam> {
        use TerrainType::*;
        match (&self, beam) {
            (NorthEastMirror, North) => vec![East],
            (NorthEastMirror, East) => vec![North],
            (NorthEastMirror, South) => vec![West],
            (NorthEastMirror, West) => vec![South],
            (NorthWestMirror, North) => vec![West],
            (NorthWestMirror, East) => vec![South],
            (NorthWestMirror, South) => vec![East],
            (NorthWestMirror, West) => vec![North],
            (NorthSouthSplitter, East | West) => vec![North, South],
            (EastWestSplitter, North | South) => vec![East, West],
            (_, _) => vec![beam.clone()],
        }
    }
}

#[derive(Debug)]
struct ContraptionMap {
    rows: Int,
    columns: Int,
    grid: Grid<Terrain>,
    visited: Points,
}

impl ContraptionMap {
    fn parse(input: Input) -> ContraptionMap {
        let input = input.lines().map(|s| s.to_string()).collect();
        let map = ContraptionMap::from_strings(&input);
        map
    }

    fn from_strings(input: &Vec<String>) -> ContraptionMap {
        let grid: Grid<Terrain> = input
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().map(|c| Terrain::from_char(c)).collect::<Vec<_>>())
            .collect();
        let rows = grid.len() as Int;
        let columns = grid.first().unwrap().len() as Int;
        ContraptionMap {
            rows,
            columns,
            grid,
            visited: Points::new(),
        }
    }

    fn get_terrain(&self, point: &Point) -> &Terrain {
        &self.grid[point.y as usize][point.x as usize]
    }

    fn get_terrain_mut(&mut self, point: &Point) -> &mut Terrain {
        &mut self.grid[point.y as usize][point.x as usize]
    }

    fn get_energy_map(&self) -> String {
        self.grid
            .iter()
            .map(|v| {
                v.iter()
                    .map(|t| if t.energized { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn get_terrain_map(&self) -> String {
        self.grid
            .iter()
            .map(|v| v.iter().map(|t| t.the_type.to_char()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn within_grid(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.columns && point.y >= 0 && point.y < self.rows
    }

    fn shoot_beam(&mut self, from: &Point, beam: Beam) {
        let mut points: Points = Points::new();
        debug!(false, "shoot({:?}, {:?})", from, beam);
        if self.within_grid(from) {
            self.visited.insert(*from);
            let current_terrain: &Terrain = self.get_terrain(from);
            if !current_terrain.beams.contains(&beam) {
                let current_terrain = self.get_terrain_mut(from);
                current_terrain.beams.push(beam.clone());
                if !current_terrain.energized {
                    current_terrain.energized = true;
                }
                points.insert(*from);
                let redirected_beams = current_terrain.the_type.redirect(&beam);
                for redirected_beam in redirected_beams {
                    let next: Point = from.move_to(&redirected_beam);
                    self.shoot_beam(&next, redirected_beam);
                }
            }
        }

        debug!(false, "shoot({:?}, {:?}) -> {:?}", from, beam, points);
    }

    fn get_amount_of_energized_tiles(&mut self, point: &Point, beam: Beam) -> Int {
        self.grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|t| {
                t.energized = false;
                t.beams.clear();
            })
        });
        self.visited.clear();
        self.shoot_beam(&point, beam);
        self.visited.len() as Int
    }

    fn get_most_amount_of_energized_tiles(&mut self) -> Int {
        let mut most = 0;
        for i in 0..self.columns {
            most = max(
                most,
                self.get_amount_of_energized_tiles(&Point::new(i, 0), South),
            );
            most = max(
                most,
                self.get_amount_of_energized_tiles(&Point::new(i, self.rows - 1), North),
            );
        }
        for i in 0..self.rows {
            most = max(
                most,
                self.get_amount_of_energized_tiles(&Point::new(0, i), East),
            );
            most = max(
                most,
                self.get_amount_of_energized_tiles(&Point::new(0, self.columns - 1), West),
            );
        }
        most
    }
}

#[derive(Default)]
pub struct Problem {}

impl Solution for Problem {
    fn year(&self) -> Year {
        2023
    }
    fn day(&self) -> Day {
        16
    }
    fn expect_part_one(&self) -> Answer {
        6906
    }
    fn expect_part_two(&self) -> Answer {
        7330
    }

    define_examples! {
        (
            r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
            ",
            Expect::PartsOneAndTwo(46, 51),
        )
    }

    fn solve_part_one(&self, input: Input, is_example: bool) -> Answer {
        let mut map = ContraptionMap::parse(input);
        map.shoot_beam(&Point::new(0, 0), East);
        let amount_of_energized_tiles = map.get_amount_of_energized_tiles(&Point::new(0, 0), East);
        if is_example {
            let terrain_map = map.get_terrain_map();
            let energy_map = map.get_energy_map();
            println!("Terrain map:\n{}\n", terrain_map);
            println!("Energy map:\n{}\n", energy_map);
            let example_expected_energized_map = vec![
                "######....",
                ".#...#....",
                ".#...#####",
                ".#...##...",
                ".#...##...",
                ".#...##...",
                ".#..####..",
                "########..",
                ".#######..",
                ".#...#.#..",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n");
            test!(example_expected_energized_map, energy_map, "energy_map");
        }
        amount_of_energized_tiles
    }

    fn solve_part_two(&self, input: Input, _is_example: bool) -> Answer {
        let mut map = ContraptionMap::parse(input);
        let most_amount_energized = map.get_most_amount_of_energized_tiles();
        most_amount_energized
    }
}

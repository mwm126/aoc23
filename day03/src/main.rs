use std::cmp::{max, min};
use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("Solution part 1:{}", part_sum(&input));
    println!("Solution part 2:{}", gear_prod(&input));
    Ok(())
}

fn part_sum(input: &str) -> u32 {
    let schematic = parse_schematic(input);
    schematic.count_parts()
}

fn gear_prod(input: &str) -> u32 {
    let schematic = parse_schematic(input);
    schematic.sum_gears()
}

struct Schematic {
    grid: Vec<String>,
    parts: Vec<Part>,
    geargrid: HashMap<(usize,usize), Gear>,
}

#[derive(Clone)]
struct Part {
    id: u32,
    start: (usize, usize),
    length: usize,
}

struct Gear {
    parts: Vec<Part>
}

impl Gear {
    fn score(&self) -> u32 {
        if self.parts.len() == 2 {
            self.parts[0].id * self.parts[1].id
        } else {
            0
        }
    }

    fn add_part(&mut self, part:Part) {
        self.parts.push(part)
    }
}

fn parse_schematic(input: &str) -> Schematic {
    let mut grid = Vec::new();
    let mut parts = Vec::new();
    let mut geargrid = HashMap::<(usize,usize), Gear>::new();
    for (ii, line) in input.lines().enumerate() {
        let mut grid_line = "".to_string();
        let mut current_num = "".to_string();
        let mut part_start: usize = 0;
        for (jj, cc) in line.chars().enumerate() {
            grid_line.push(cc);
            if cc.is_ascii_digit() {
                if current_num.is_empty() {
                    part_start = jj;
                    current_num = "".to_string();
                }
                current_num.push(cc)
            } else if !current_num.is_empty() {
                let part = Part {
                    id: current_num.parse::<u32>().unwrap(),
                    start: (ii, part_start),
                    length: (jj - part_start),
                };
                parts.push(part);
                current_num = "".to_string();
            }
            if cc == '.' {
                continue;
            }
        }

        if !current_num.is_empty() {
            let part = Part {
                id: current_num.parse::<u32>().unwrap(),
                start: (ii, part_start),
                length: (line.len() - part_start),
            };
            parts.push(part);
        }

        grid.push(grid_line)
    }

    for part in &parts {
        let (i0, j0) = part.start;
        let lower = max(i0, 1) - 1;
        let upper = min(i0 + 1, grid.len() - 1);
        for ii in lower..=upper {
            let jlower = max(j0, 1) - 1;
            let jupper = min(j0 + part.length, grid[0].len() - 1);
            for jj in jlower..=jupper {
                let cc = grid[ii].chars().nth(jj).unwrap();
                if cc == '*' {
                    let key = (ii,jj);
                    if !geargrid.contains_key(&key) {
                    let gear = Gear {
                        parts: Vec::new()
                    };
                        geargrid.insert(key, gear);
                    }
                    let gg = geargrid.get_mut(&key).unwrap();
                    gg.add_part(part.clone())
                }
            }
        }
    }

    Schematic { grid, parts, geargrid }
}

impl Schematic {
    fn count_parts(&self) -> u32 {
        let mut sum = 0;
        for part in &self.parts {
            if self.symbol_adjacent(part) {
                sum += part.id;
            }
        }
        sum
    }

    fn sum_gears(&self) -> u32 {
        let mut sum = 0;
        for (_, gear) in &self.geargrid {
                sum += gear.score()
        }
        sum
    }

    fn symbol_adjacent(&self, part: &Part) -> bool {
        let (i0, j0) = part.start;
        let lower = max(i0, 1) - 1;
        let upper = min(i0 + 1, self.grid.len() - 1);
        for ii in lower..=upper {
            let jlower = max(j0, 1) - 1;
            let jupper = min(j0 + part.length, self.grid[0].len() - 1);
            for jj in jlower..=jupper {
                let cc = self.grid[ii].chars().nth(jj).unwrap();
                if !cc.is_ascii_digit() && cc != '.' {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_parse() {
        let schematic = parse_schematic(EXAMPLE_INPUT);
        assert_eq!(schematic.parts[0].id, 467);
        assert_eq!(schematic.parts[1].id, 114);
        assert_eq!(schematic.parts[2].id, 35);
        assert_eq!(schematic.parts[3].id, 633);
        assert_eq!(schematic.parts[4].id, 617);
        assert_eq!(schematic.parts[5].id, 58);
    }

    #[test]
    fn test_symbol() {
        let ss = parse_schematic(EXAMPLE_INPUT);
        assert!(ss.symbol_adjacent(&ss.parts[0]));
        assert!(!ss.symbol_adjacent(&ss.parts[1]));
        assert!(ss.symbol_adjacent(&ss.parts[2]));
        assert!(ss.symbol_adjacent(&ss.parts[3]));
        assert!(ss.symbol_adjacent(&ss.parts[4]));
        assert!(!ss.symbol_adjacent(&ss.parts[5]));
    }

    #[test]
    fn test_part_sum() {
        let result = part_sum(EXAMPLE_INPUT);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_gear_prod() {
        let result = gear_prod(EXAMPLE_INPUT);
        assert_eq!(result, 467835);
    }
}

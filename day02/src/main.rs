use std::cmp::max;
use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("Solution part 1:{}", possibles(&input));
    println!("Solution part 2:{}", possibles2(&input));
    Ok(())
}

pub fn possibles(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (game_id, peak_r, peak_g, peak_b) = parse_line(line);
        if peak_r <= 12 && peak_g <= 13 && peak_b <= 14 {
            sum += game_id
        }
    }
    sum
}

pub fn possibles2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += game_power(line)
    }
    sum
}

pub fn parse_line(input: &str) -> (u32, u32, u32, u32) {
    let (game_id, rest) = input.split_once(':').expect("Invalid input");
    let id: u32 = game_id[5..].parse().expect("Bad ID");
    let mut rr = 0;
    let mut gg = 0;
    let mut bb = 0;
    for triple in rest.split(';') {
        for cube_count in triple.split(',') {
            let mut split = cube_count.split_whitespace();
            let count = split.next().unwrap();
            let color = split.next().unwrap();
            let count: u32 = count.parse().expect("Bad count");
            match color {
                "red" => rr = max(rr, count),
                "blue" => bb = max(bb, count),
                "green" => gg = max(gg, count),
                _ => {}
            }
        }
    }
    (id, rr, gg, bb)
}

pub fn game_power(line: &str) -> u32 {
    let (_, rr, gg, bb) = parse_line(line);
    rr * gg * bb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            (1, 4, 2, 6)
        );
        assert_eq!(
            parse_line("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            (2, 1, 3, 4)
        );
    }

    #[test]
    fn test_power() {
        assert_eq!(
            game_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );
        assert_eq!(
            game_power("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            12
        );

        assert_eq!(
            game_power("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            1560
        );
        assert_eq!(
            game_power("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            630
        );
        assert_eq!(
            game_power("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            36
        );
    }

    #[test]
    fn example() {
        let example_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = possibles(example_input);
        assert_eq!(result, 8);
        let result = possibles2(example_input);
        assert_eq!(result, 2286);
    }
}

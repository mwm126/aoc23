use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("Solution part 1:{}", calibrate(&input));
    println!("Solution part 2:{}", calibrate2(&input));
    Ok(())
}

pub fn calibrate(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += cal(line);
    }
    sum
}

pub fn calibrate2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += cal2(line);
    }
    sum
}

pub fn cal(input: &str) -> u32 {
    let mut nums = vec![];
    for c in input.chars() {
        if let Some(num) = c.to_digit(10) {
            nums.push(num);
        }
    }
    nums.last().unwrap() + 10 * nums.first().unwrap()
}

fn digit_map() -> HashMap<String, u32> {
    HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
        ("zero".to_string(), 0),
        ("0".to_string(), 0),
        ("1".to_string(), 1),
        ("2".to_string(), 2),
        ("3".to_string(), 3),
        ("4".to_string(), 4),
        ("5".to_string(), 5),
        ("6".to_string(), 6),
        ("7".to_string(), 7),
        ("8".to_string(), 8),
        ("9".to_string(), 9),
    ])
}

fn furst(input: &str) -> u32 {
    for ii in 0..input.len() {
        let digits = digit_map();
        for (key, val) in &digits {
            if input[ii..].starts_with(key) {
                return *val;
            }
        }
    }
    panic!("INVALID INPUT {input}", input = input)
}

fn lassed(input: &str) -> u32 {
    for ii in (0..input.len()).rev() {
        let digits = digit_map();
        for (key, val) in &digits {
            if input[ii..].starts_with(key) {
                return *val;
            }
        }
    }
    panic!("INVALID INPUT {input}", input = input)
}

pub fn cal2(input: &str) -> u32 {
    lassed(input) + 10 * furst(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line() {
        assert_eq!(cal("1abc2"), 12);
        assert_eq!(cal("pqr3stu8vwx"), 38);
        assert_eq!(cal("a1b2c3d4e5f"), 15);
        assert_eq!(cal("treb7uchet"), 77);
    }

    #[test]
    fn line2() {
        assert_eq!(cal2("two1nine"), 29);
        assert_eq!(cal2("eightwothree"), 83);
        assert_eq!(cal2("abcone2threexyz"), 13);
        assert_eq!(cal2("xtwone3four"), 24);
        assert_eq!(cal2("4nineeightseven2"), 42);
        assert_eq!(cal2("zoneight234"), 14);
        assert_eq!(cal2("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_rere() {
        assert_eq!(furst("two1nine"), 2);
        assert_eq!(furst("eightwothree"), 8);
        assert_eq!(furst("abcone2threexyz"), 1);
        assert_eq!(furst("xtwone3four"), 2);
        assert_eq!(furst("4nineeightseven2"), 4);
        assert_eq!(furst("zoneight234"), 1);
        assert_eq!(furst("7pqrstsixteen"), 7);

        assert_eq!(lassed("two1nine"), 9);
        assert_eq!(lassed("eightwothree"), 3);
        assert_eq!(lassed("abcone2threexyz"), 3);
        assert_eq!(lassed("xtwone3four"), 4);
        assert_eq!(lassed("4nineeightseven2"), 2);
        assert_eq!(lassed("zoneight234"), 4);
        assert_eq!(lassed("7pqrstsixteen"), 6);
    }

    #[test]
    fn example() {
        let example_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = calibrate(example_input);
        assert_eq!(result, 142);
    }
}

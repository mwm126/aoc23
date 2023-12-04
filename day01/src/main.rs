use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let output = calibrate(input);
    println!("{}", output);
    Ok(())
}

pub fn calibrate(input: String) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += cal(line);
    }
    sum
}

pub fn calibrate2(input: String) -> u32 {
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

pub fn cal2(input: &str) -> u32 {
    let mut nums = vec![];
    for c in input.chars() {
        if let Some(num) = c.to_digit(10) {
            nums.push(num);
        }
    }
    nums.last().unwrap() + 10 * nums.first().unwrap()
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

    // #[test]
    // fn line2() {
    //     assert_eq!(cal2("two1nine"), 29);
    //     assert_eq!(cal2("eightwothree"), 83);
    //     assert_eq!(cal2("abcone2threexyz"), 13);
    //     assert_eq!(cal2("xtwone3four"), 24);
    //     assert_eq!(cal2("4nineeightseven2"), 42);
    //     assert_eq!(cal2("zoneight234"), 14);
    //     assert_eq!(cal2("7pqrstsixteen"), 76);
    // }

    #[test]
    fn example() {
        let example_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = calibrate(example_input.into());
        assert_eq!(result, 142);
    }
}

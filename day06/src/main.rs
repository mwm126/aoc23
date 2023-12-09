use std::io;
use std::iter::zip;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("Solution part 1:{}", prod_ways_to_win(&input));
    println!("Solution part 2:{}", prod_ways_to_win2(&input));
    Ok(())
}

fn prod_ways_to_win(input: &str) -> usize {
    let mut prod = 1;
    let races = parse_races(input);
    for (time, dist) in races {
        prod *= ways_to_win(time, dist)
    }
    prod
}

fn prod_ways_to_win2(input: &str) -> usize {
    let mut prod = 1;
    let races = parse_races2(input);
    for (time, dist) in races {
        prod *= ways_to_win(time, dist)
    }
    prod
}

fn ways_to_win(time: usize, dist: usize) -> usize {
    // hold for t seconds
    // speed = t*1 = t
    // duration = time-t
    // max_dist = speed*duration = t*(time-t)
    // for how many values of t is t*(time-t) > dist?
    let mut ways = 0;
    for t in 1..time {
        if t * (time - t) > dist {
            ways += 1;
        }
    }
    ways
}

fn to_num(nn: &str) -> usize {
    nn.parse().expect("Invalid num")
}

fn parse_races(input: &str) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    let mut lines = input.lines();
    let times = lines.next().expect("bad input").split_whitespace().skip(1);
    let distances = lines.next().expect("bad input").split_whitespace().skip(1);
    for (time, dist) in zip(times, distances) {
        results.push((to_num(time), to_num(dist)))
    }
    results
}

fn parse_races2(input: &str) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    let mut lines = input.lines();
    let times:String = lines.next().expect("bad input").split_once(':').expect("bad parse").1.to_string();
    let dists:String = lines.next().expect("bad input").split_once(':').expect("bad parse").1.to_string();
    let time = times.replace(" ", "");
    let dist = dists.replace(" ", "");
    results.push((to_num(&time), to_num(&dist)));
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
    Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_card_score() {
        assert_eq!(ways_to_win(7, 9), 4);
        assert_eq!(ways_to_win(15, 40), 8);
        assert_eq!(ways_to_win(30, 200), 9);
    }

    #[test]
    fn test_ways_to_win() {
        let result = prod_ways_to_win(EXAMPLE_INPUT);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_ways_to_win2() {
        let result = prod_ways_to_win2(EXAMPLE_INPUT);
        assert_eq!(result, 71503);
    }
}

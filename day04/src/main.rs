use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("Solution part 1:{}", card_points(&input));
    println!("Solution part 2:{}", card_count(&input));
    Ok(())
}

fn card_points(input: &str) -> usize {
    let mut sum = 0;
    let (cards, _) = parse_cards(input);
    for card in cards {
        sum += card.score()
    }
    sum
}

fn card_count(input: &str) -> usize {
    let mut sum = 0;
    let (cards, copies) = parse_cards(input);
    for (ii, _card) in cards.iter().enumerate() {
        let copy_count = copies[&ii];
        sum += copy_count;
    }
    sum
}

fn parse_cards(input: &str) -> (Vec<Card>, HashMap<usize, usize>) {
    let mut cards = Vec::new();
    let mut copies: HashMap<usize, usize> = HashMap::new();
    for (ii, _) in input.lines().enumerate() {
        copies.insert(ii, 1);
    }

    for (ii, line) in input.lines().enumerate() {
        let card = parse_line(line);
        for jj in ii + 1..ii + 1 + card.win_count() {
            *copies.get_mut(&jj).unwrap() += copies[&ii];
        }
        cards.push(card);
    }
    (cards, copies)
}

fn parse_line(line: &str) -> Card {
    let (_card_name, nums) = line.split_once(':').expect("Invalid line");
    let (wins, nums) = nums.split_once('|').expect("Invalid numbers");
    fn to_num(nn: &str) -> usize {
        nn.parse().expect("Invalid num")
    }
    let winners = wins.split_whitespace().map(to_num).collect();
    let numbers = nums.split_whitespace().map(to_num).collect();
    Card { winners, numbers }
}

struct Card {
    winners: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn win_count(&self) -> usize {
        let mut wins = HashSet::new();
        for win in &self.winners {
            wins.insert(win);
        }
        self.numbers
            .iter()
            .map(|nn| if wins.contains(nn) { 1 } else { 0 })
            .sum()
    }

    fn score(&self) -> usize {
        let match_count = self.win_count();
        if match_count > 0 {
            2_usize.pow(match_count as u32 - 1)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_card_score() {
        assert_eq!(
            parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").score(),
            8
        );
        assert_eq!(
            parse_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").score(),
            2
        );
        assert_eq!(
            parse_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").score(),
            2
        );
        assert_eq!(
            parse_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").score(),
            1
        );
        assert_eq!(
            parse_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").score(),
            0
        );
        assert_eq!(
            parse_line("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").score(),
            0
        );
    }

    #[test]
    fn test_card_points() {
        let result = card_points(EXAMPLE_INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_card_count() {
        let result = card_count(EXAMPLE_INPUT);
        assert_eq!(result, 30);
    }
}

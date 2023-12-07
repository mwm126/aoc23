use std::collections::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("Solution part 1:{}", card_points(&input));
    Ok(())
}

fn card_points(input: &str) -> usize {
    let mut sum = 0;
    let cards = parse_cards(input);
    for card in cards {
        sum += card.score()
    }
    sum
}

fn parse_cards(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    for line in input.lines() {
        let card = parse_line(line);
        cards.push(card);
    }
    cards
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
    fn score(&self) -> usize {
        let mut wins = HashSet::new();
        for win in &self.winners {
            wins.insert(win);
        }
        let match_count: u32 = self
            .numbers
            .iter()
            .map(|nn| if wins.contains(nn) { 1 } else { 0 })
            .sum();
        if match_count > 0 {
            2_usize.pow(match_count - 1)
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
}

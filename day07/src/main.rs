use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("Solution part 1:{}", total_winnings(&input));
    Ok(())
}

fn total_winnings(input: &str) -> usize {
    let mut winnings = 0;
    let mut hands = parse_camelcard(input);
    hands.sort();
    for (ii, hand) in hands.iter().enumerate() {
        winnings += (ii + 1) * hand.bid;
    }
    winnings
}

fn parse_camelcard(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').expect("bad input");
            let rank = find_rank(cards);
            Hand {
                rank,
                cards: cards
                    .replace('A', "Z")
                    .replace('K', "Y")
                    .replace('Q', "X")
                    .replace('J', "W")
                    .replace('T', "V")
                    .to_string(),
                bid: bid.parse().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    rank: Rank,
    cards: String,
    bid: usize,
}

fn find_rank(cards: &str) -> Rank {
    let mut ranx = HashMap::<char, usize>::new();
    for cc in cards.chars() {
        let count = ranx.entry(cc).or_insert(0);
        *count += 1;
    }
    let mut rrs = Vec::<usize>::new();
    for rr in ranx.values() {
        rrs.push(*rr)
    }
    rrs.sort();
    rrs.reverse();
    if rrs[0] == 5 {
        Rank::Five
    } else if rrs[0] == 4 {
        Rank::Four
    } else if rrs[0] == 3 && rrs[1] == 2 {
        Rank::Full
    } else if rrs[0] == 3 {
        Rank::Three
    } else if rrs[0] == 2 && rrs[1] == 2 {
        Rank::Two
    } else if rrs[0] == 2 {
        Rank::One
    } else {
        Rank::High
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Rank {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    const EXAMPLE_INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_card_score() {
        let hands = parse_camelcard(EXAMPLE_INPUT);
        assert_eq!(hands[0].rank, Rank::One);
        assert_eq!(hands[1].rank, Rank::Three);
        assert_eq!(hands[2].rank, Rank::Two);
        assert_eq!(hands[3].rank, Rank::Two);
        assert_eq!(hands[4].rank, Rank::Three);

        assert_eq!(hands[0].cmp(&hands[3]), Ordering::Less);
        assert_eq!(hands[3].cmp(&hands[2]), Ordering::Less);
        assert_eq!(hands[2].cmp(&hands[1]), Ordering::Less);
        assert_eq!(hands[1].cmp(&hands[4]), Ordering::Less);
    }

    #[test]
    fn test_total_winnings() {
        let result = total_winnings(EXAMPLE_INPUT);
        assert_eq!(result, 6440);
    }
}

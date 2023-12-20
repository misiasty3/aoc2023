struct Hand {
    h: [u8; 5],
    h_type: u8,
    bid: u32,
}

fn solution(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let sp = line
                .split_once(' ')
                .expect("Failed to separate hand and bid");
            let hand = to_hand(sp.0);
            Hand {
                h: hand,
                h_type: hand_to_type(&hand),
                bid: sp.1.parse().expect("Failed to parse number"),
            }
        })
        .collect();
    hands.sort_unstable_by(|h1, h2| h1.h_type.cmp(&h2.h_type).then(h1.h.cmp(&h2.h)).reverse());
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid as usize)
        .sum()
}

fn to_hand(str: &str) -> [u8; 5] {
    let mut out = [0; 5];
    for (i, ch) in str.chars().enumerate() {
        out[i] = match ch {
            'A' => 0,
            'K' => 1,
            'Q' => 2,
            'J' => 12,
            'T' => 3,
            num => 4 + 9 - num.to_digit(10).expect("Unexpected card type") as u8,
        }
    }
    out
}

fn hand_to_type(hand: &[u8; 5]) -> u8 {
    let mut all_cards = [0; 15];
    let mut max = 0;
    let mut max_pos = 0;
    let mut jokers = 0;
    for card in hand {
        all_cards[*card as usize] += 1;
        let new_v = all_cards[*card as usize];
        if new_v > max {
            max = new_v;
            max_pos = *card as usize;
        }
        if card == &12 {
            jokers += 1;
        }
    }
    all_cards[max_pos] += jokers;

    return if all_cards.contains(&5) {
        0
    } else if all_cards.contains(&4) {
        1
    } else if all_cards.contains(&3) && all_cards.contains(&2) {
        2
    } else if all_cards.contains(&3) {
        3
    } else {
        match all_cards.iter().filter(|n| n == &&2).count() {
            0 => 6,
            1 => 5,
            2 => 4,
            _ => panic!(),
        }
    };
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solution(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_test() {
        let input = include_str!("example1.txt");
        assert_eq!(solution(input), 5905);
    }

    #[test]
    fn type_test() {
        let hands = ["32T3K", "KK677", "T55J5", "KTJJT", "QQQJA"];
        let types = [5, 4, 1, 1, 1];
        for i in 0..5 {
            assert_eq!(hand_to_type(&to_hand(&hands[i])), types[i]);
        }
    }

    #[test]
    fn custom_test() {
        let input = "\
1111J 134
2222A 212
JJJJJ 333
";
        //         let input = "\
        // 1111J 134
        // ";
        assert_eq!(solution(input), 134 * 3 + 333 * 2 + 212 * 1);
        // assert_eq!(solution(input), 134);
    }
}

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
            'J' => 3,
            'T' => 4,
            num => 5 + 9 - num.to_digit(10).expect("Unexpected card type") as u8,
        }
    }
    out
}

fn hand_to_type(hand: &[u8; 5]) -> u8 {
    let mut all_cards = [0; 15];
    for card in hand {
        all_cards[*card as usize] += 1;
    }
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
        assert_eq!(solution(input), 6440);
    }

    #[test]
    fn type_test() {
        let hands = [
            "AAAAA", "AA8AA", "23332", "TTT98", "23432", "A23A4", "23456",
        ];
        let types = [0, 1, 2, 3, 4, 5, 6];
        for i in 0..7 {
            assert_eq!(hand_to_type(&to_hand(&hands[i])), types[i]);
        }
    }
}

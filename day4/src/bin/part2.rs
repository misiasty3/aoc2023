use std::collections::HashSet;

fn solution(input: &str) -> usize {
    let mut game_cards: Vec<(usize, usize)> =
        input.lines().map(|line| (process_game(line), 1)).collect();

    let mut i = 0;
    while i < game_cards.len() {
        // dbg!(i);
        // dbg!(game_cards.clone());
        for card in i + 1..i + 1 + game_cards[i].0 {
            if card >= game_cards.len() {
                break;
            }
            game_cards[card].1 += game_cards[i].1;
        }
        i += 1;
    }
    game_cards.iter().map(|card| card.1).sum()
}

fn process_game(game: &str) -> usize {
    let data: String = game.chars().skip_while(|c| *c != ':').skip(1).collect();
    let (lucky_nums, nums) = data.split_once('|').expect("Wrong data!");

    let lucky_nums_set: HashSet<u32> =
        HashSet::from_iter(lucky_nums.split_whitespace().map(|num| parse(num)));

    let winning_nums = nums
        .split_whitespace()
        .filter(|num| lucky_nums_set.contains(&parse(num)))
        .count();

    winning_nums
}

fn parse(num: &str) -> u32 {
    match num.parse() {
        Ok(a) => a,
        _ => {
            eprintln!("Cant parse number: \"{num}\"");
            panic!()
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solution(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("example.txt");
        assert_eq!(solution(input), 30);
    }
}

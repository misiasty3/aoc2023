use std::collections::HashSet;

fn solution(input: &str) -> u32 {
    input.lines().map(|line| process_game(line)).sum()
}

fn process_game(game: &str) -> u32 {
    let data: String = game.chars().skip_while(|c| *c != ':').skip(1).collect();
    let (lucky_nums, nums) = data.split_once('|').expect("Wrong data!");

    // dbg!(lucky_nums);
    // dbg!(nums);

    let lucky_nums_set: HashSet<u32> =
        HashSet::from_iter(lucky_nums.split_whitespace().map(|num| parse(num)));

    let winning_nums = nums
        .split_whitespace()
        .filter(|num| lucky_nums_set.contains(&parse(num)))
        .count();

    match winning_nums {
        0 => 0,
        _ => 1 << winning_nums - 1,
    }
    // dbg!(1 << winning_nums - 1)
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
        assert_eq!(solution(input), 13);
    }
}

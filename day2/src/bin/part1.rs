fn solution(x: &str) -> usize {
    x.lines()
        .enumerate()
        .filter(|game| is_game_possible(game.1))
        .map(|game| game.0 + 1)
        .sum::<usize>()
}

fn is_game_possible(mut game: &str) -> bool {
    const RED: usize = 12;
    const GREEN: usize = 13;
    const BLUE: usize = 14;

    let start = game.find(':').unwrap() + 2;
    game = &game[start..];

    let colors = game.split(';').map(|game| game.split(',')).flatten();
    for color in colors {
        let (num, color) = color.trim().split_once(' ').unwrap();
        let amount: usize = num.parse().unwrap();
        if match color {
            "red" => amount > RED,
            "green" => amount > GREEN,
            "blue" => amount > BLUE,
            _ => panic!(),
        } {
            return false;
        }
    }

    true
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
        let example = include_str!("example1.txt");
        assert_eq!(solution(example), 8);
    }
}

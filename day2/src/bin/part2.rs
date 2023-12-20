use std::cmp;

fn solution(x: &str) -> usize {
    x.lines().map(|game| lest_possible(game)).sum::<usize>()
}

fn lest_possible(mut game: &str) -> usize {
    let mut red: usize = 0;
    let mut green: usize = 0;
    let mut blue: usize = 0;

    let start = game.find(':').unwrap() + 2;
    game = &game[start..];

    let colors = game.split(';').map(|game| game.split(',')).flatten();
    for color in colors {
        let (num, color) = color.trim().split_once(' ').unwrap();
        let amount: usize = num.parse().unwrap();
        match color {
            "red" => red = cmp::max(red, amount),
            "green" => green = cmp::max(green, amount),
            "blue" => blue = cmp::max(blue, amount),
            _ => panic!(),
        }
    }

    red * green * blue
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
        assert_eq!(solution(example), 2286);
    }
}

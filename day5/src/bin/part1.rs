fn solution(input: &str) -> usize {
    // let mut lines = input.lines();
    // let mut seeds_it = lines.next().unwrap().split_whitespace();
    let (seeds_str, rest) = input.split_once('\n').expect("Failed to find seeds");
    let (_, rest) = rest.split_once('\n').unwrap();

    let assosiations = rest
        .split("\n\n")
        .map(|line| line.lines().skip(1).collect::<Vec<_>>())
        .map(|a| dbg!(a))
        .count();
    0
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
        let input = include_str!("example1.txt");
        assert_eq!(solution(input), 35);
    }
}

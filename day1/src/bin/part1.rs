fn solution(x: &str) -> u32 {
    let mut sum = 0;
    for line in x.lines() {
        assert_ne!(line, "");
        let mut num = line.chars().filter_map(|c| c.to_digit(10));
        let f = num.next().unwrap();
        let l = num.last().unwrap_or(f);
        sum += 10 * f + l;
    }
    sum
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
        let sol = solution(input);
        assert_eq!(sol, 142);
    }
}

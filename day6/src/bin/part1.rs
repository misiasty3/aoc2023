fn solution(input: &str) -> usize {
    let (time_str, distance_str) = input
        .split_once('\n')
        .expect("No time/distance distinction");
    let (time_vec, distance_vec) = (parse(time_str), parse(distance_str));
    let it = time_vec.iter().zip(distance_vec.iter());
    it.map(|(time, record_dist)| {
        (0..*time)
            .map(|possible_t| (time - possible_t) * possible_t)
            .filter(|my_dist| my_dist > record_dist)
            .count()
    })
    .product()
}

fn parse(line: &str) -> Vec<usize> {
    let mut line_it = line.chars().skip_while(|ch| *ch != ':');
    line_it.next();
    let line_trimmed: String = line_it.collect();
    line_trimmed
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
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
        assert_eq!(solution(input), 288);
    }
}

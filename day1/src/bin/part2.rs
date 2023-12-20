use std::collections::HashMap;

fn solution(x: &str) -> u32 {
    let mut sum = 0;

    for line in x.lines() {
        let mut numbers = vec![];
        let mut i = 0;
        while i < line.len() {
            let num_check = is_number(&line[i..line.len()]);
            if let Some(num_len) = num_check {
                numbers.push(to_number(&line[i..i + num_len]));
                i += 1;
                continue;
            }
            let c = line.chars().nth(i).unwrap();
            if let Some(digit) = c.to_digit(10) {
                numbers.push(digit);
            }
            i += 1;
        }

        let mut it = numbers.iter();
        let f = it.next().unwrap();
        let l = it.last().unwrap_or(f);
        sum += 10 * f + l;
    }
    sum
}

fn is_number(x: &str) -> Option<usize> {
    const NUM3: [&str; 3] = ["one", "two", "six"];
    const NUM4: [&str; 3] = ["four", "five", "nine"];
    const NUM5: [&str; 3] = ["three", "seven", "eight"];

    if x.len() >= 3 && NUM3.contains(&&x[0..3]) {
        Some(3)
    } else if x.len() >= 4 && NUM4.contains(&&x[0..4]) {
        Some(4)
    } else if x.len() >= 5 && NUM5.contains(&&x[0..5]) {
        Some(5)
    } else {
        None
    }
}

fn to_number(x: &str) -> u32 {
    let nums = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    *nums.get(x).unwrap()
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
        let input = include_str!("example2.txt");
        let sol = solution(input);
        assert_eq!(sol, 281);
    }
}

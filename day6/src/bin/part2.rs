fn solution(input: &str) -> usize {
    let (time_str, dist_str) = input
        .split_once('\n')
        .expect("Failed to split input into time and distance");
    let time = parse(time_str);
    let dist = parse(dist_str);
    binary_search(time, dist)
}

fn binary_search(time: usize, dist: usize) -> usize {
    let mut start = 0;
    let mut end = time;

    while start != end {
        let guess = (start + end) / 2;
        if (time - guess) * guess > dist {
            end = guess;
        } else {
            start = guess + 1;
        }
    }

    let ans_start = end;

    start = ans_start;
    end = time;
    while start != end {
        let guess = (start + end) / 2;
        if (time - guess) * guess > dist {
            start = guess + 1;
        } else {
            end = guess;
        }
    }

    let ans_end = end;
    ans_end - ans_start
}

// fn is_best(dist: usize, time: usize, guess: usize) -> bool {
//     (time - guess) * guess > dist
// }

fn parse(line: &str) -> usize {
    let num: String = line.split_whitespace().skip(1).collect();
    num.parse().expect("Failed number parsing")
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
        assert_eq!(solution(input), 71503);
    }
}

fn solution(input: &str) -> i64 {
    // let mut i = 0;
    input
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse().expect("Failed to parse num"))
                .collect();
            // dbg!(i += 1);
            find_prev(nums)
        })
        .sum()
}

fn find_prev(seq: Vec<i64>) -> i64 {
    let mut rows = vec![seq];
    while !rows.last().unwrap().iter().all(|num| *num == 0) {
        let new_vec: Vec<i64> = rows
            .last()
            .unwrap()
            .windows(2)
            .map(|a| a[1] - a[0])
            .collect();
        rows.push(new_vec);
    }

    // dbg!(rows.clone());

    let mut prev = 0;
    for seq in rows.iter().rev() {
        prev = seq.first().unwrap() - prev;
    }
    prev
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
        assert_eq!(solution(input), 2);
    }

    #[test]
    fn test2() {
        let input = "0 7 18 31 55 131 378 1093 2953 7398 17321 38257 80352 161507 312219 582730 1053014 1845605 3139788 5182383 8283893";
        assert_eq!(solution(input), 0);
    }
}

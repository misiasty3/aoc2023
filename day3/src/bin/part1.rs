fn solution(x: &str) -> u32 {
    let lines = x.lines();
    let board = lines
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    sum_board(board)
}

fn sum_board(board: Vec<Vec<char>>) -> u32 {
    let mut sum = 0;

    for (y, row) in board.iter().enumerate() {
        let mut num = String::new();

        for (x, ch) in row.iter().enumerate() {
            if ch.is_ascii_digit() {
                num.push(*ch);
            } else if !num.is_empty() {
                if is_valid_num((x - num.len()) as i32, x as i32, y as i32, &board) {
                    sum += num.parse::<u32>().expect("error while parsing");
                }
                num.clear();
            }
        }

        let x = board[0].len();
        if !num.is_empty() && is_valid_num((x - num.len()) as i32, x as i32, y as i32, &board) {
            sum += num.parse::<u32>().expect("error while parsing");
        }
    }

    sum
}

fn is_valid_num(sta: i32, end: i32, y: i32, b: &Vec<Vec<char>>) -> bool {
    // dbg!(format!("sta: {sta}, end: {end}, y: {y}"));
    for index in sta..end {
        if is_sym(index, y - 1, &b) || is_sym(index, y + 1, &b) {
            return true;
        }
    }

    let other_chars = [
        is_sym(sta - 1, y - 1, &b),
        is_sym(sta - 1, y, &b),
        is_sym(sta - 1, y + 1, &b),
        is_sym(end, y - 1, &b),
        is_sym(end, y, &b),
        is_sym(end, y + 1, &b),
    ];

    other_chars.iter().any(|c| *c)
}

fn is_sym(x: i32, y: i32, board: &Vec<Vec<char>>) -> bool {
    y >= 0
        && y < board.len() as i32
        && x >= 0
        && x < board[0].len() as i32
        && board[y as usize][x as usize] != '.'
        && !board[y as usize][x as usize].is_ascii_digit()
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
        assert_eq!(solution(example), 4361);
    }

    #[test]
    fn symbol_test() {
        let board1 = "....";
        let board2 = ".123";
        let board3 = "#...";
        let b = vec![board1, board2, board3]
            .iter()
            .map(|board| board.chars().collect::<Vec<char>>())
            .collect();
        assert!(is_valid_num(1, 4, 1, &b))
    }

    #[test]
    fn own_test() {
        let test = "123..321
        @......@
        456..654
        ";
        assert_eq!(solution(test), 1554);
    }
}

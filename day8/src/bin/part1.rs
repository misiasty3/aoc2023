use std::collections::HashMap;

fn solution(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().expect("No instructions");
    lines.next();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for node in lines {
        let (place, dirs_str) = node.split_once(" =").expect("Map entry not found");
        let mut dirs = dirs_str.split_once(',').expect("Cant separete directions");
        dirs.0 = &dirs.0[2..5];
        dirs.1 = &dirs.1[1..4];

        map.insert(place, dirs);
    }

    // dbg!(map.clone());

    let mut pos = "AAA";
    for (i, ins) in instructions.chars().cycle().enumerate() {
        // dbg!(pos);
        pos = match ins {
            'L' => map.get(pos).expect("Entry not found in map").0,
            'R' => map.get(pos).expect("Entry not found in map").1,
            _ => panic!(),
        };

        if pos == "ZZZ" {
            return i + 1;
        }
    }
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
        assert_eq!(solution(input), 2);
    }
    #[test]
    fn test2() {
        let input = include_str!("example2.txt");
        assert_eq!(solution(input), 6);
    }
}

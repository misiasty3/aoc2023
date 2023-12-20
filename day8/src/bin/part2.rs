use num::Integer;
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

    map.keys()
        .filter(|key| key.ends_with('A'))
        .map(|start| find_end(start, instructions, &map))
        .inspect(|e| println!("{e}"))
        .reduce(|acc, e| acc.lcm(&e)) // TODO: Write my own lcm implementation
        .unwrap()
}

fn find_end(start: &str, insts: &str, map: &HashMap<&str, (&str, &str)>) -> usize {
    let mut pos = start;
    let inst_loop = insts.chars().cycle();

    for (i, inst) in inst_loop.enumerate() {
        let new_pos = map.get(pos).expect("Value in map not found");
        pos = match inst {
            'L' => new_pos.0,
            'R' => new_pos.1,
            _ => panic!(),
        };

        if pos.ends_with('Z') {
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
        let input = include_str!("example3.txt");
        assert_eq!(solution(input), 6);
    }
}

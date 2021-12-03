use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Debug)]
enum Direction {
    Forward,
    Up,
    Down,
    Unknown,
}

#[derive(PartialEq, Debug)]
struct Command {
    direction: Direction,
    distance: isize,
}

#[derive(PartialEq, Debug)]
struct Position {
    horizontal: isize,
    depth: isize,
}

fn parse_direction(direction: &str) -> Direction {
    match direction {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => Direction::Unknown,
    }
}

fn parse_command(line: &str) -> Command {
    let mut parts = line.splitn(2, " ");
    let direction = parse_direction(parts.next().unwrap());
    Command {
        direction: direction,
        distance: parts.next().unwrap().parse().unwrap(),
    }
}

fn apply_command(initial_position: Position, command: Command) -> Position {
    match command {
        Command {
            direction: Direction::Forward,
            distance,
        } => Position {
            horizontal: initial_position.horizontal + distance,
            ..initial_position
        },
        Command {
            direction: Direction::Down,
            distance,
        } => Position {
            depth: initial_position.depth + distance,
            ..initial_position
        },
        Command {
            direction: Direction::Up,
            distance,
        } => Position {
            depth: initial_position.depth - distance,
            ..initial_position
        },
        _ => initial_position,
    }
}

fn read_input(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let commands: Vec<Command> = read_input("input")
        .iter()
        .map(|c| parse_command(c))
        .collect();
    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };
    for command in commands {
        position = apply_command(position, command);
    }
    println!(
        "After following these instructions, you would have a horizontal position of {} and a depth of {}. (Multiplying these together produces {}.)",
        position.horizontal, position.depth, position.horizontal * position.depth,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_direction() {
        assert_eq!(parse_direction("up"), Direction::Up);
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(
            parse_command("forward 10"),
            Command {
                direction: Direction::Forward,
                distance: 10
            }
        );
    }

    #[test]
    fn test_apply_command() {
        let initial_position = Position {
            horizontal: 0,
            depth: 0,
        };
        let command = Command {
            direction: Direction::Down,
            distance: 10,
        };
        let expected_position = Position {
            horizontal: 0,
            depth: 10,
        };
        assert_eq!(apply_command(initial_position, command), expected_position);
    }

    #[test]
    fn test_read_input() {
        assert_eq!(read_input("input")[0], "forward 4");
    }
}

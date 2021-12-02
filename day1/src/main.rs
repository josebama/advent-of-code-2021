use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn count_increases(input: &[isize]) -> i32 {
    let mut increases = 0;
    let mut previous: Option<&isize> = None;
    for value in input.iter() {
        if let Some(prev) = previous {
            if prev < value {
                increases += 1;
            }
        }
        previous = Some(value);
    }
    increases
}

fn read_input(path: &str) -> Vec<isize> {
    let file = File::open(path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|read| {
            read.expect("Failed to read line")
                .parse::<isize>()
                .expect("Failed to parse line")
        })
        .collect()
}

fn sum_windows(input: &[isize], window_size: usize) -> Vec<isize> {
    input
        .windows(window_size)
        .map(|window| window.iter().sum())
        .collect()
}

fn main() {
    let input = read_input("day1/input");
    let increases = count_increases(&input);
    println!(
        "There are {} measurements that are larger than the previous measurement",
        increases
    );
    let windowed_increases = count_increases(&sum_windows(&input, 3));
    println!(
        "There are {} sums that are larger than the previous sum",
        windowed_increases
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases() {
        assert_eq!(count_increases(&[1, 2, 3, 2, 3, 1]), 3);
    }

    #[test]
    fn test_read_input() {
        assert_eq!(read_input("input")[0], 178);
    }

    #[test]
    fn test_sum_windows() {
        assert_eq!(sum_windows(&[1, 2, 3, 4], 2), [3, 5, 7]);
    }
}

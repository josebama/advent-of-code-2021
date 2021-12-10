fn get_bit(num: u16, pos: u8) -> u8 {
    ((num >> pos) % 2) as u8
}

#[test]
fn test_get_bit() {
    assert_eq!(get_bit(0b1010, 0), 0);
    assert_eq!(get_bit(0b1010, 1), 1);
    assert_eq!(get_bit(0b1010, 2), 0);
    assert_eq!(get_bit(0b1010, 3), 1);
}

fn bit_array(bits: &[u8]) -> u16 {
    let mut result: u16 = 0;
    for bit in bits {
        result = result << 1;
        result += *bit as u16;
    }
    result
}

#[test]
fn test_bit_array() {
    assert_eq!(bit_array(&[1, 0, 1, 0]), 0b1010);
}

fn get_gamma(reads: &[u16], row_len: u8) -> u16 {
    let mut ones: Vec<u16> = vec![0; row_len as usize];
    for read in reads {
        for pos in 0..row_len {
            ones[pos as usize] += get_bit(*read, pos) as u16;
        }
    }
    let half = (reads.len() / 2) as u16;
    let bits = ones
        .iter()
        .rev()
        .map(|&count| (count > half) as u8)
        .collect::<Vec<u8>>();
    bit_array(&bits)
}

#[test]
fn test_get_gamma() {
    assert_eq!(get_gamma(&[0b0100, 0b0001, 0b0111], 4), 0b0101)
}

fn get_epsilon(gamma: u16, row_len: u8) -> u16 {
    ((1 << row_len) - 1) - gamma
}

#[test]
fn test_get_epsilon() {
    assert_eq!(get_epsilon(0b0101, 4), 0b1010);
}

fn main() {
    let input = include_str!("../input");
    let rows = input.split_ascii_whitespace().collect::<Vec<&str>>();
    let row_len = rows[0].len() as u8;
    let reads = rows
        .iter()
        .map(|row| u16::from_str_radix(row, 2).unwrap())
        .collect::<Vec<u16>>();
    let gamma = get_gamma(&reads, row_len);
    let epsilon = get_epsilon(gamma, row_len);
    print!(
        "Gamma {}. Epsilon {}. Mult: {}",
        gamma,
        epsilon,
        gamma as u32 * epsilon as u32
    );
}

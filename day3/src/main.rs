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

fn count_ones(reads: &[u16], pos: u8) -> u16 {
    reads
        .iter()
        .fold(0, |count, &val| get_bit(val, pos) as u16 + count)
}

#[test]
fn test_count_ones() {
    let reads = [0b0100, 0b0001, 0b0111];
    assert_eq!(count_ones(&reads, 0), 2);
    assert_eq!(count_ones(&reads, 1), 1);
    assert_eq!(count_ones(&reads, 2), 2);
    assert_eq!(count_ones(&reads, 3), 0);
}

fn get_gamma(reads: &[u16], row_len: u8) -> u16 {
    let mut ones = Vec::new();
    for pos in 0..row_len {
        ones.push(count_ones(reads, pos));
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

fn filter_reads(reads: Vec<u16>, pos: u8, bit: u8) -> Vec<u16> {
    reads
        .into_iter()
        .filter(|&read| get_bit(read, pos) == bit)
        .collect::<Vec<u16>>()
}

#[test]
fn test_filter_reads() {
    assert_eq!(
        filter_reads([0b00, 0b01, 0b10, 0b11].to_vec(), 0, 1),
        [0b01, 0b11]
    )
}

fn get_rating(reads: &[u16], row_len: u8, get_bit_criteria: &dyn Fn(f32, f32) -> u8) -> u16 {
    let mut result = reads.to_vec();
    for i in (0..row_len).rev() {
        let ones = count_ones(&result, i);
        let half = result.len() as f32 / 2.0;
        let bit_criteria = get_bit_criteria(ones as f32, half);
        result = filter_reads(result, i, bit_criteria);
        if result.len() == 1 {
            return result[0];
        }
    }
    panic!("Rating not found!")
}

fn get_o2_bit_criteria(ones: f32, half: f32) -> u8 {
    (ones >= half) as u8
}

fn get_co2_bit_criteria(ones: f32, half: f32) -> u8 {
    (ones < half) as u8
}

fn get_o2_rating(reads: &[u16], row_len: u8) -> u16 {
    get_rating(reads, row_len, &get_o2_bit_criteria)
}

fn get_co2_rating(reads: &[u16], row_len: u8) -> u16 {
    get_rating(reads, row_len, &get_co2_bit_criteria)
}

#[test]
fn test_get_o2_rating() {
    assert_eq!(
        get_o2_rating(&[0b0100, 0b0001, 0b0111, 0b1011, 0b1111], 4),
        0b0111
    )
}

#[test]
fn test_get_co2_rating() {
    assert_eq!(
        get_co2_rating(&[0b0100, 0b0001, 0b0111, 0b1011, 0b1111], 4),
        0b1011
    )
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
    println!(
        "Gamma {}. Epsilon {}. Mult: {}",
        gamma,
        epsilon,
        gamma as u32 * epsilon as u32
    );
    let o2 = get_o2_rating(&reads, row_len);
    let co2 = get_co2_rating(&reads, row_len);
    println!("O2 {}. CO2 {}. Mult: {}", o2, co2, o2 as u32 * co2 as u32);
}

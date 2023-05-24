pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();
    let mut file = file.peekable();

    let length = file.peek().unwrap().len();
    let mut bits_sums = Vec::new();
    bits_sums.resize(length, 0);
    let mut report_len = 0;

    for line in file {
        for (i, bit) in line.chars().enumerate() {
            let bit = bit.to_digit(10).unwrap();
            bits_sums[i] += bit;
        }
        report_len += 1;
    }

    let bits_sums = bits_sums.iter().map(|x| {
        let x = *x;
        if x > report_len - x {
            1
        } else {
            0
        }
    });

    let mut gamma = 0;
    let mut mask = 0;
    for bit in bits_sums {
        gamma = gamma << 1;
        gamma += bit;
        mask = mask << 1;
        mask += 1;
    }
    let epsilon_rate = gamma ^ mask;

    gamma * epsilon_rate
}

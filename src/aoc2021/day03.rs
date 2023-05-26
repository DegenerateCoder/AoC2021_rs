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

struct Instruction {
    num: String,
    filtered: bool,
}

pub fn part02(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();
    let file = file.map(|x| Instruction {
        num: x.to_string(),
        filtered: false,
    });
    let mut file: Vec<Instruction> = file.collect();

    // find oxygen generator rating
    let mut mcb_pos = 0;
    while instructions_left(&file) > 1 {
        let mcb = find_mcb_lcb(&file, mcb_pos, true);
        let file_iter = file.iter_mut().filter(|x| !x.filtered);
        for instruction in file_iter {
            let bit = instruction.num.chars().skip(mcb_pos).next().unwrap();
            let bit = bit.to_digit(10).unwrap();

            if bit != mcb {
                instruction.filtered = true;
            }
        }
        mcb_pos += 1;
    }

    let o_g_r = {
        let mut mcb = file.iter().filter(|x| !x.filtered);
        let mcb = mcb.next().unwrap();
        u32::from_str_radix(&mcb.num, 2).unwrap()
    };

    // reset
    file.iter_mut().for_each(|x| x.filtered = false);

    // find CO2 scrubber rating
    let mut lcb_pos = 0;
    while instructions_left(&file) > 1 {
        let lcb = find_mcb_lcb(&file, lcb_pos, false);
        let file_iter = file.iter_mut().filter(|x| !x.filtered);
        for instruction in file_iter {
            let bit = instruction.num.chars().skip(lcb_pos).next().unwrap();
            let bit = bit.to_digit(10).unwrap();

            if bit != lcb {
                instruction.filtered = true;
            }
        }
        lcb_pos += 1;
    }

    let mut lcb = file.iter().filter(|x| !x.filtered);
    let lcb = lcb.next().unwrap();
    let co2_s_r = u32::from_str_radix(&lcb.num, 2).unwrap();

    o_g_r * co2_s_r
}

fn find_mcb_lcb(instructions: &Vec<Instruction>, pos: usize, mcb: bool) -> u32 {
    let instructions = instructions.iter();
    let instructions = instructions.filter(|x| !x.filtered);

    let mut sum = 0;
    let mut instruction_num = 0;
    for instruction in instructions {
        let bit = instruction.num.chars().skip(pos).next();
        let bit = bit.unwrap().to_digit(10).unwrap();
        sum += bit;
        instruction_num += 1;
    }

    let expresion = if mcb {
        sum >= instruction_num - sum
    } else {
        sum < instruction_num - sum
    };

    if expresion {
        1
    } else {
        0
    }
}

fn instructions_left(instructions: &Vec<Instruction>) -> u32 {
    let mut left = 0;
    for instruction in instructions {
        if !instruction.filtered {
            left += 1;
        }
    }

    left
}

pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut file = file.lines();

    let bits_transmission = file.next().unwrap();
    let bits_transmission = decode_hex(bits_transmission);

    let mut i = 0;

    let packet_version = &bits_transmission[i..i + 3];
    let packet_version = bits_to_dec(packet_version);
    i += 3;

    let type_id = &bits_transmission[i..i + 3];
    let type_id = bits_to_dec(type_id);
    i += 3;

    let mut version_sum = packet_version;
    if type_id != 4 {
        process_sub_packet_ver(&mut i, &bits_transmission, &mut version_sum);
    }

    version_sum
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut file = file.lines();

    let bits_transmission = file.next().unwrap();
    let bits_transmission = decode_hex(bits_transmission);

    let mut i = 0;

    // skip packet version
    i += 3;

    let type_id = &bits_transmission[i..i + 3];
    let type_id = bits_to_dec(type_id);
    i += 3;

    if type_id != 4 {
        let extracted_sub_packets = extract_sub_packets(&mut i, &bits_transmission);
        return process_operator_packet(type_id, extracted_sub_packets);
    }
    panic!("Only literal value packet");
}

fn process_sub_packet_ver(i: &mut usize, bits_transmission: &Vec<bool>, version_sum: &mut usize) {
    let length_type_id = {
        if bits_transmission[*i] {
            11
        } else {
            15
        }
    };
    *i += 1;
    let to_process = &bits_transmission[*i..*i + length_type_id];
    let to_process = bits_to_dec(to_process);
    *i += length_type_id;

    let start_i = *i;
    let mut processed_sub_packets = 0;
    loop {
        let packet_version = &bits_transmission[*i..*i + 3];
        let packet_version = bits_to_dec(packet_version);
        *version_sum += packet_version;
        *i += 3;

        let type_id = &bits_transmission[*i..*i + 3];
        let type_id = bits_to_dec(type_id);
        *i += 3;

        if type_id != 4 {
            process_sub_packet_ver(i, bits_transmission, version_sum);
        } else {
            id_4_lit_val(i, &bits_transmission);
        }
        processed_sub_packets += 1;
        if length_type_id == 15 {
            let processed = *i - start_i;
            if processed >= to_process {
                break;
            }
        } else {
            if processed_sub_packets >= to_process {
                break;
            }
        }
    }
}

fn decode_hex(bits_transmission: &str) -> Vec<bool> {
    let mut binary = Vec::new();
    let bits_transmission = bits_transmission.chars();
    for hex_char in bits_transmission {
        let hex_char: u8 = hex_char.to_digit(16).unwrap().try_into().unwrap();
        for bit in (0..4).rev() {
            let mask = 1 << bit;
            binary.push((hex_char & mask) != 0);
        }
    }

    binary
}

fn id_4_lit_val(i: &mut usize, bits_transmission: &Vec<bool>) -> usize {
    let mut lit_val: Vec<bool> = Vec::new();
    loop {
        let prefix = bits_transmission[*i];
        *i += 1;

        let bit_group = &bits_transmission[*i..*i + 4];
        *i += 4;

        bit_group.iter().for_each(|bit| lit_val.push(*bit));
        if !prefix {
            break;
        }
    }

    bits_to_dec(&lit_val)
}

fn bits_to_dec(bits: &[bool]) -> usize {
    let mut dec: usize = 0;
    for (i, bit) in bits.iter().rev().enumerate() {
        if *bit {
            dec += 1 << i;
        }
    }

    dec
}

fn extract_sub_packets(i: &mut usize, bits_transmission: &Vec<bool>) -> Vec<usize> {
    let mut sub_packets_values: Vec<usize> = Vec::new();
    let length_type_id = {
        if bits_transmission[*i] {
            11
        } else {
            15
        }
    };
    *i += 1;
    let to_process = &bits_transmission[*i..*i + length_type_id];
    let to_process = bits_to_dec(to_process);
    *i += length_type_id;

    let start_i = *i;
    let mut processed_sub_packets = 0;
    loop {
        // skip packet version
        *i += 3;

        let type_id = &bits_transmission[*i..*i + 3];
        let type_id = bits_to_dec(type_id);
        *i += 3;

        if type_id != 4 {
            let extracted_sub_packets = extract_sub_packets(i, bits_transmission);
            sub_packets_values.push(process_operator_packet(type_id, extracted_sub_packets));
        } else {
            sub_packets_values.push(id_4_lit_val(i, &bits_transmission));
        }
        processed_sub_packets += 1;
        if length_type_id == 15 {
            let processed = *i - start_i;
            if processed >= to_process {
                break;
            }
        } else {
            if processed_sub_packets >= to_process {
                break;
            }
        }
    }

    sub_packets_values
}

fn process_operator_packet(type_id: usize, sub_packets_values: Vec<usize>) -> usize {
    match type_id {
        0 => return sub_packets_values.iter().sum(),
        1 => {
            let mut product = 1;
            sub_packets_values.iter().for_each(|x| product *= x);
            return product;
        }
        2 => return *(sub_packets_values.iter().min().unwrap()),
        3 => return *(sub_packets_values.iter().max().unwrap()),
        5 => {
            if sub_packets_values[0] > sub_packets_values[1] {
                return 1;
            } else {
                return 0;
            }
        }
        6 => {
            if sub_packets_values[0] < sub_packets_values[1] {
                return 1;
            } else {
                return 0;
            }
        }
        7 => {
            if sub_packets_values[0] == sub_packets_values[1] {
                return 1;
            } else {
                return 0;
            }
        }
        _ => return 0,
    }
}

#[allow(dead_code)]
fn print_bits(bits_transmission: &Vec<bool>) {
    for bit in bits_transmission {
        if *bit == true {
            print!("1");
        } else {
            print!("0");
        }
    }
    println!("");
}

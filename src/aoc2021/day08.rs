pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let signal_notes = file.map(|x| x.split('|').skip(1).next().unwrap().trim());

    let mut instances_of_digits = 0;
    for note in signal_notes {
        note.split(' ').for_each(|x| match x.len() {
            2 | 4 | 3 | 7 => instances_of_digits += 1,
            _ => (),
        });
    }

    instances_of_digits
}

#[derive(Clone, Copy)]
struct SegmentActivationCount {
    name: char,
    count: usize,
}

pub fn part02(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let signal_notes = file.map(|x| x.split('|'));

    let mut sum: u32 = 0;
    for mut note in signal_notes {
        let signal_pattern = note.next().unwrap().trim();
        let output_values = note.next().unwrap().trim();

        let segments: [char; 8] = decipher_segments(signal_pattern);

        let output_values = output_values
            .split(' ')
            .map(|x| decode_signal(x, &segments));

        let output_value: String = output_values.collect();
        let output_value: u32 = output_value.parse().unwrap();

        sum += output_value;
    }

    sum
}

fn decode_signal(signal: &str, segments: &[char; 8]) -> char {
    let signal_len = signal.len();

    let value: char = match signal_len {
        2 => '1',
        3 => '7',
        7 => '8',
        4 => '4',
        5 => {
            if !signal.contains(segments[5]) {
                '2'
            } else if signal.contains(segments[2]) {
                '3'
            } else {
                '5'
            }
        }
        6 => {
            if !signal.contains(segments[4]) {
                '0'
            } else if signal.contains(segments[6]) {
                '6'
            } else {
                '9'
            }
        }
        _ => ' ',
    };

    value
}

fn decipher_segments(signal_pattern: &str) -> [char; 8] {
    let mut segments_ac: [SegmentActivationCount; 7] = [SegmentActivationCount {
        name: ' ',
        count: 0,
    }; 7];
    ('a'..='g')
        .zip(segments_ac.iter_mut())
        .for_each(|(name, segment)| segment.name = name);

    segments_ac
        .iter_mut()
        .for_each(|x| x.count = signal_pattern.chars().filter(|c| *c == x.name).count());

    let mut segments: [char; 8] = [' '; 8];

    segments_ac.iter().for_each(|x| match x {
        SegmentActivationCount { name, count: 6 } => segments[3] = *name,
        SegmentActivationCount { name, count: 9 } => segments[5] = *name,
        SegmentActivationCount { name, count: 4 } => segments[6] = *name,
        _ => (),
    });

    segments[1] = {
        let one = signal_pattern
            .split(' ')
            .filter(|x| x.len() == 2)
            .next()
            .unwrap();
        let seven = signal_pattern
            .split(' ')
            .filter(|x| x.len() == 3)
            .next()
            .unwrap();

        let mut diff = seven.chars().filter(|c| !one.contains(*c));

        diff.next().unwrap()
    };

    segments[2] = {
        let sac_8 = segments_ac.iter().filter(|x| x.count == 8);
        let mut sac_8 = sac_8.filter(|x| !(x.name == segments[1]));

        sac_8.next().unwrap().name
    };

    segments[4] = {
        let four = signal_pattern
            .split(' ')
            .filter(|x| x.len() == 4)
            .next()
            .unwrap();
        let mut diff = four
            .chars()
            .filter(|c| *c != segments[2] && *c != segments[3] && *c != segments[5]);

        diff.next().unwrap()
    };

    segments[7] = ('a'..='g')
        .filter(|c| !segments.contains(c))
        .next()
        .unwrap();

    segments
}

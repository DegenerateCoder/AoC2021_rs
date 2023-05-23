struct Command {
    direction: String,
    value: u32,
}

pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();
    let file = file.map(|x| {
        let mut command = x.split(' ');
        let direction = command.next().unwrap().to_string();
        let value: u32 = command.next().unwrap().parse().unwrap();
        Command { direction, value }
    });
    let mut x = 0;
    let mut y = 0;
    for command in file {
        match command.direction.as_str() {
            "forward" => x += command.value,
            "down" => y += command.value,
            "up" => y -= command.value,
            _ => (),
        }
    }

    x * y
}

pub fn part02(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();
    let file = file.map(|x| {
        let mut command = x.split(' ');
        let direction = command.next().unwrap().to_string();
        let value: u32 = command.next().unwrap().parse().unwrap();
        Command { direction, value }
    });
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for command in file {
        match command.direction.as_str() {
            "forward" => {
                x += command.value;
                y += command.value * aim;
            }
            "down" => aim += command.value,
            "up" => aim -= command.value,
            _ => (),
        }
    }

    x * y
}

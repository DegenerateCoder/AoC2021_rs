pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut file = file.lines();

    let mut depth_increases = 0;
    let mut old_depth: u32 = file.next().unwrap().parse().unwrap();
    for line in file {
        let currrent_depth: u32 = line.parse().unwrap();
        if currrent_depth > old_depth {
            depth_increases += 1;
        }
        old_depth = currrent_depth;
    }

    depth_increases
}

pub fn part02(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let mut window_end = file.map(|x| x.parse::<u32>().unwrap());
    let mut window_start = window_end.clone();

    let mut old_window_sum: u32 =
        window_end.next().unwrap() + window_end.next().unwrap() + window_end.next().unwrap();
    let mut depth_increases = 0;

    for line in window_end {
        let current_window_sum = old_window_sum + line - window_start.next().unwrap();
        if current_window_sum > old_window_sum {
            depth_increases += 1;
        }
        old_window_sum = current_window_sum;
    }

    depth_increases
}

pub fn part02_v2(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let mut window_start = file.map(|x| x.parse::<u32>().unwrap());
    let mut window_end = window_start.clone();

    window_start.next();
    window_start.next();
    window_start.next();
    let mut depth_increases = 0;

    for line in window_start {
        if line > window_end.next().unwrap() {
            depth_increases += 1;
        }
    }

    depth_increases
}

#[derive(Debug)]
struct LineSegment {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let file = file.map(|x| x.split("->").map(|y| y.split(',')).flatten());
    let line_segments = file.map(|mut x| LineSegment {
        x1: x.next().unwrap().trim().parse().unwrap(),
        y1: x.next().unwrap().trim().parse().unwrap(),
        x2: x.next().unwrap().trim().parse().unwrap(),
        y2: x.next().unwrap().trim().parse().unwrap(),
    });
    let line_segments: Vec<LineSegment> = line_segments.collect();

    let x_max = line_segments
        .iter()
        .map(|x| if x.x1 > x.x2 { x.x1 } else { x.x2 })
        .max()
        .unwrap() as usize
        + 1;
    let y_max = line_segments
        .iter()
        .map(|x| if x.y1 > x.y2 { x.y1 } else { x.y2 })
        .max()
        .unwrap() as usize
        + 1;

    let mut diagram: Vec<Vec<u32>> = Vec::new();
    diagram.resize(y_max, vec![0; x_max]);

    let line_segments = line_segments
        .iter()
        .filter(|x| x.x1 == x.x2 || x.y1 == x.y2);
    for line in line_segments {
        if line.x1 == line.x2 {
            let x = line.x1;
            let y_end = line.y1.max(line.y2);
            let y_start = line.y1.min(line.y2);
            for y in y_start..=y_end {
                diagram[y as usize][x as usize] += 1;
            }
        } else if line.y1 == line.y2 {
            let y = line.y1;
            let x_end = line.x1.max(line.x2);
            let x_start = line.x1.min(line.x2);
            for x in x_start..=x_end {
                diagram[y as usize][x as usize] += 1;
            }
        }
    }

    let overlapp_count = diagram.iter().flatten().filter(|x| **x > 1).count() as u32;

    overlapp_count
}

pub fn part02(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let file = file.map(|x| x.split("->").map(|y| y.split(',')).flatten());
    let line_segments = file.map(|mut x| LineSegment {
        x1: x.next().unwrap().trim().parse().unwrap(),
        y1: x.next().unwrap().trim().parse().unwrap(),
        x2: x.next().unwrap().trim().parse().unwrap(),
        y2: x.next().unwrap().trim().parse().unwrap(),
    });
    let line_segments: Vec<LineSegment> = line_segments.collect();

    let x_max = line_segments
        .iter()
        .map(|x| if x.x1 > x.x2 { x.x1 } else { x.x2 })
        .max()
        .unwrap() as usize
        + 1;
    let y_max = line_segments
        .iter()
        .map(|x| if x.y1 > x.y2 { x.y1 } else { x.y2 })
        .max()
        .unwrap() as usize
        + 1;

    let mut diagram: Vec<Vec<u32>> = Vec::new();
    diagram.resize(y_max, vec![0; x_max]);

    for line in line_segments {
        let x1 = line.x1 as i32;
        let x2 = line.x2 as i32;
        let y1 = line.y1 as i32;
        let y2 = line.y2 as i32;

        let x_step = (x2 - x1).signum();
        let y_step = (y2 - y1).signum();

        let mut x = x1;
        let mut y = y1;
        loop {
            diagram[y as usize][x as usize] += 1;
            x += x_step;
            y += y_step;
            if (x == x2 + x_step) && (y == y2 + y_step) {
                break;
            }
        }
    }

    let overlapp_count = diagram.iter().flatten().filter(|x| **x > 1).count() as u32;

    overlapp_count
}




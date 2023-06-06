struct Dot {
    x: usize,
    y: usize,
}

pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();
    let file: Vec<&str> = file.collect();

    let dots_coordinates = file
        .iter()
        .filter(|x| x.contains(','))
        .map(|x| x.split(','));
    let dots_coordinates = dots_coordinates.map(|mut x| Dot {
        x: x.next().unwrap().parse().unwrap(),
        y: x.next().unwrap().parse().unwrap(),
    });
    let dots_coordinates: Vec<Dot> = dots_coordinates.collect();

    let max_x = dots_coordinates.iter().map(|x| x.x).max().unwrap() + 1;
    let max_y = dots_coordinates.iter().map(|x| x.y).max().unwrap() + 1;
    let mut dot_map: Vec<Vec<bool>> = vec![vec![false; max_x]; max_y];
    dots_coordinates
        .iter()
        .for_each(|x| dot_map[x.y][x.x] = true);

    let mut fold_instructions = file
        .iter()
        .filter(|x| x.contains('='))
        .map(|x| x.replace("fold along ", ""));

    let fold_instruction = fold_instructions.next().unwrap();
    let mut fold_instruction = fold_instruction.split('=');
    let fold_direction = fold_instruction.next().unwrap();
    let fold_value: usize = fold_instruction.next().unwrap().parse().unwrap();

    dot_map = fold(dot_map, fold_direction, fold_value);

    let mut visible_dot_count = 0;
    dot_map.iter().flatten().for_each(|x| {
        if *x {
            visible_dot_count += 1
        }
    });

    visible_dot_count
}

pub fn part02(file_path: &str) -> String {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();
    let file: Vec<&str> = file.collect();

    let dots_coordinates = file
        .iter()
        .filter(|x| x.contains(','))
        .map(|x| x.split(','));
    let dots_coordinates = dots_coordinates.map(|mut x| Dot {
        x: x.next().unwrap().parse().unwrap(),
        y: x.next().unwrap().parse().unwrap(),
    });
    let dots_coordinates: Vec<Dot> = dots_coordinates.collect();

    let max_x = dots_coordinates.iter().map(|x| x.x).max().unwrap() + 1;
    let max_y = dots_coordinates.iter().map(|x| x.y).max().unwrap() + 1;
    let mut dot_map: Vec<Vec<bool>> = vec![vec![false; max_x]; max_y];
    dots_coordinates
        .iter()
        .for_each(|x| dot_map[x.y][x.x] = true);

    let fold_instructions = file
        .iter()
        .filter(|x| x.contains('='))
        .map(|x| x.replace("fold along ", ""));

    let mut fold_lim_x = 0;
    let mut fold_lim_y = 0;
    for fold_instruction in fold_instructions {
        let mut fold_instruction = fold_instruction.split('=');
        let fold_direction = fold_instruction.next().unwrap();
        let fold_value: usize = fold_instruction.next().unwrap().parse().unwrap();
        if fold_direction == "y" {
            fold_lim_y = fold_value;
        } else {
            fold_lim_x = fold_value;
        }

        dot_map = fold(dot_map, fold_direction, fold_value);
    }

    let mut code = String::new();
    for (y, row) in dot_map.iter().enumerate() {
        for (x, dot) in row.iter().enumerate() {
            if x < fold_lim_x && y < fold_lim_y {
                if *dot {
                    code.push('#');
                } else {
                    code.push('.');
                }
            }
        }
        if y < fold_lim_y {
            code.push('\n');
        }
    }

    code
}

fn fold(dot_map: Vec<Vec<bool>>, fold_direction: &str, fold_value: usize) -> Vec<Vec<bool>> {
    let mut dot_map_folded: Vec<Vec<bool>> = vec![vec![false; dot_map.len()]; dot_map[0].len()];
    for (y, row) in dot_map.iter().enumerate() {
        for (x, dot) in row.iter().enumerate() {
            if *dot {
                if fold_direction == "y" {
                    if y > fold_value {
                        let new_y = fold_value - (y - fold_value);
                        dot_map_folded[new_y][x] = *dot;
                    } else {
                        dot_map_folded[y][x] = *dot;
                    }
                } else {
                    if x > fold_value {
                        let new_x = fold_value - (x - fold_value);
                        dot_map_folded[y][new_x] = *dot;
                    } else {
                        dot_map_folded[y][x] = *dot;
                    }
                }
            }
        }
    }

    dot_map_folded
}

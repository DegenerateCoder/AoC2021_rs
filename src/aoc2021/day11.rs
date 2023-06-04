struct EnergyLevel {
    value: u32,
    flashed: bool,
}

pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();
    let file = file.map(|x| {
        x.chars()
            .map(|c| EnergyLevel {
                value: c.to_digit(10).unwrap(),
                flashed: false,
            })
            .collect()
    });

    let mut energy_levels_map: Vec<Vec<EnergyLevel>> = file.collect();

    let mut falsh_count = 0;
    for _ in 1..=100 {
        increas_energy_levels(&mut energy_levels_map);
        simulate(&mut energy_levels_map, &mut falsh_count);
    }

    falsh_count
}

pub fn part02(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();
    let file = file.map(|x| {
        x.chars()
            .map(|c| EnergyLevel {
                value: c.to_digit(10).unwrap(),
                flashed: false,
            })
            .collect()
    });

    let mut energy_levels_map: Vec<Vec<EnergyLevel>> = file.collect();

    let mut step = 0;
    let max_y = energy_levels_map.len();
    let max_x = energy_levels_map[0].len();
    loop {
        step += 1;
        let mut falsh_count = 0;
        increas_energy_levels(&mut energy_levels_map);
        simulate(&mut energy_levels_map, &mut falsh_count);
        if (falsh_count as usize) == max_x * max_y {
            break;
        }
    }

    step
}

fn increas_energy_levels(energy_levels_map: &mut Vec<Vec<EnergyLevel>>) {
    for row in energy_levels_map {
        for mut energy_level in row {
            energy_level.value += 1;
            energy_level.flashed = false;
        }
    }
}

fn simulate(energy_levels_map: &mut Vec<Vec<EnergyLevel>>, flash_count: &mut u32) {
    let mut x_f = 0;
    let mut y_f = 0;
    let mut flash = false;

    'row: for (y, row) in energy_levels_map.iter().enumerate() {
        y_f = y;
        for (x, energy_level) in row.iter().enumerate() {
            x_f = x;
            if energy_level.flashed == false && energy_level.value > 9 {
                flash = true;
                *flash_count += 1;
                break 'row;
            }
        }
    }

    if flash {
        simulate_flash(x_f, y_f, energy_levels_map);
        simulate(energy_levels_map, flash_count);
    } else {
        return;
    }
}

struct Vec2 {
    x: i32,
    y: i32,
}

fn simulate_flash(x: usize, y: usize, energy_levels_map: &mut Vec<Vec<EnergyLevel>>) {
    energy_levels_map[y][x].flashed = true;
    energy_levels_map[y][x].value = 0;
    let x = x as i32;
    let y = y as i32;
    let max_y = energy_levels_map.len() as i32;
    let max_x = energy_levels_map[0].len() as i32;
    let adjacent_locations: [Vec2; 8] = [
        Vec2 { x: x - 1, y: y - 1 },
        Vec2 { x, y: y - 1 },
        Vec2 { x: x + 1, y: y - 1 },
        Vec2 { x: x - 1, y },
        Vec2 { x: x + 1, y },
        Vec2 { x: x - 1, y: y + 1 },
        Vec2 { x, y: y + 1 },
        Vec2 { x: x + 1, y: y + 1 },
    ];
    let adjacent_locations = adjacent_locations.iter().filter(|pos| {
        if pos.x < 0 || pos.y < 0 {
            false
        } else if pos.x >= max_x || pos.y >= max_y {
            false
        } else {
            true
        }
    });
    adjacent_locations.for_each(|pos| {
        let x = pos.x as usize;
        let y = pos.y as usize;
        if energy_levels_map[y][x].flashed == false {
            energy_levels_map[y][x].value += 1;
        }
    })
}

pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let heightmap: Vec<Vec<u32>> = file
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let max_y = heightmap.len();
    let max_x = heightmap[0].len();
    let mut risk_levels_sum = 0;

    for (y, row) in heightmap.iter().enumerate() {
        for (x, hight_value) in row.iter().enumerate() {
            let mut adjacent_locations: [u32; 4] = [0; 4];
            adjacent_locations[0] = {
                if y == 0 {
                    9
                } else {
                    heightmap[y - 1][x]
                }
            };
            adjacent_locations[1] = {
                let y = y + 1;
                if y >= max_y {
                    9
                } else {
                    heightmap[y][x]
                }
            };
            adjacent_locations[2] = {
                if x == 0 {
                    9
                } else {
                    heightmap[y][x - 1]
                }
            };
            adjacent_locations[3] = {
                let x = x + 1;
                if x >= max_x {
                    9
                } else {
                    heightmap[y][x]
                }
            };

            if *hight_value < *(adjacent_locations.iter().min().unwrap()) {
                risk_levels_sum += hight_value + 1;
            }
        }
    }

    risk_levels_sum
}

#[derive(Clone, Copy, PartialEq)]
struct LowPoint {
    x: usize,
    y: usize,
    value: u32,
}

pub fn part02(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let heightmap: Vec<Vec<u32>> = file
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let max_y = heightmap.len();
    let max_x = heightmap[0].len();

    let mut low_points: Vec<LowPoint> = Vec::new();
    for (y, row) in heightmap.iter().enumerate() {
        for (x, hight_value) in row.iter().enumerate() {
            let mut adjacent_locations: [u32; 4] = [0; 4];
            adjacent_locations[0] = {
                if y == 0 {
                    9
                } else {
                    heightmap[y - 1][x]
                }
            };
            adjacent_locations[1] = {
                let y = y + 1;
                if y >= max_y {
                    9
                } else {
                    heightmap[y][x]
                }
            };
            adjacent_locations[2] = {
                if x == 0 {
                    9
                } else {
                    heightmap[y][x - 1]
                }
            };
            adjacent_locations[3] = {
                let x = x + 1;
                if x >= max_x {
                    9
                } else {
                    heightmap[y][x]
                }
            };

            if *hight_value < *(adjacent_locations.iter().min().unwrap()) {
                let value = heightmap[y][x];
                low_points.push(LowPoint { x, y, value });
            }
        }
    }

    let basin_sizes = low_points.iter().map(|x| {
        let mut points: Vec<LowPoint> = Vec::new();
        calculat_basin_points(&mut points, &heightmap, x);

        points.len()
    });

    let mut basin_sizes: Vec<usize> = basin_sizes.collect();
    basin_sizes.sort_unstable();

    let mul = basin_sizes[basin_sizes.len() - 1]
        * basin_sizes[basin_sizes.len() - 2]
        * basin_sizes[basin_sizes.len() - 3];

    mul as u32
}

fn calculat_basin_points(
    points: &mut Vec<LowPoint>,
    heightmap: &Vec<Vec<u32>>,
    low_point: &LowPoint,
) {
    let LowPoint { x, y, value } = *low_point;
    let max_y = heightmap.len();
    let max_x = heightmap[0].len();

    let mut adjacent_locations: [LowPoint; 4] = [LowPoint {
        x: 0,
        y: 0,
        value: 0,
    }; 4];
    adjacent_locations[0] = {
        if y == 0 {
            LowPoint {
                x: 0,
                y: 0,
                value: 9,
            }
        } else {
            let value = heightmap[y - 1][x];
            LowPoint { x, y: y - 1, value }
        }
    };
    adjacent_locations[1] = {
        let y = y + 1;
        if y >= max_y {
            LowPoint {
                x: 0,
                y: 0,
                value: 9,
            }
        } else {
            let value = heightmap[y][x];
            LowPoint { x, y, value }
        }
    };
    adjacent_locations[2] = {
        if x == 0 {
            LowPoint {
                x: 0,
                y: 0,
                value: 9,
            }
        } else {
            let value = heightmap[y][x - 1];
            LowPoint { x: x - 1, y, value }
        }
    };
    adjacent_locations[3] = {
        let x = x + 1;
        if x >= max_x {
            LowPoint {
                x: 0,
                y: 0,
                value: 9,
            }
        } else {
            let value = heightmap[y][x];
            LowPoint { x, y, value }
        }
    };

    let basin_points = adjacent_locations
        .iter()
        .filter(|x| x.value != 9 && x.value > value);

    if !points.contains(low_point) {
        points.push(*low_point)
    }

    basin_points.for_each(|x| calculat_basin_points(points, heightmap, x));
}

use std::collections::HashMap;

pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let risk_map: Vec<Vec<usize>> = file
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    a_star(&risk_map)
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let risk_map: Vec<Vec<usize>> = file
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    a_star_2(&risk_map)
}

#[derive(Clone, Copy)]
struct Cell {
    f: usize,
    g: usize,
    h: usize,
}

fn a_star(risk_map: &Vec<Vec<usize>>) -> usize {
    let max_pos: i32 = (risk_map.len() - 1).try_into().unwrap();

    let mut open: HashMap<[usize; 2], Cell> = HashMap::new();
    let mut closed: HashMap<[usize; 2], Cell> = HashMap::new();
    open.insert([0, 0], Cell { f: 0, g: 0, h: 0 });

    let mut total_risk = 0;
    while !open.is_empty() {
        let (min_cell_key, _) = open.iter().min_by_key(|(_, value)| value.f).unwrap();
        let min_cell_key = min_cell_key.clone();
        let min_cell = open.remove(&min_cell_key).unwrap();
        closed.insert(min_cell_key, min_cell);

        let adjacent_cells = get_adjacent_cells(&min_cell_key, max_pos);
        let max_pos: usize = max_pos.try_into().unwrap();
        if adjacent_cells.contains(&[max_pos, max_pos]) {
            total_risk = min_cell.g;
            break;
        }
        let adjacent_cells = adjacent_cells
            .iter()
            .filter(|cell_pos| !closed.contains_key(*cell_pos))
            .map(|cell_pos| {
                let x = cell_pos[0];
                let y = cell_pos[1];
                let g = min_cell.g + risk_map[y][x];
                let h = x.abs_diff(min_cell_key[0]) + y.abs_diff(min_cell_key[1]);
                let f = g + h;
                (cell_pos, Cell { f, g, h })
            });
        adjacent_cells.for_each(|(cell_pos, cell)| {
            open.entry(*cell_pos)
                .and_modify(|x| {
                    if x.f > cell.f {
                        x.f = cell.f;
                        x.g = cell.g;
                        x.h = cell.h;
                    }
                })
                .or_insert(cell);
        });
    }

    let max_pos: usize = max_pos.try_into().unwrap();
    total_risk + risk_map[max_pos][max_pos]
}

fn get_adjacent_cells(cell: &[usize; 2], max_pos: i32) -> Vec<[usize; 2]> {
    let x: i32 = cell[0].try_into().unwrap();
    let y: i32 = cell[1].try_into().unwrap();

    let adjacent_pos_offset = [[0, -1], [0, 1], [-1, 0], [1, 0]];
    let mut adjacent_pos = Vec::new();

    for [x_offset, y_offset] in adjacent_pos_offset.iter() {
        let x = x + x_offset;
        let y = y + y_offset;
        if !(x < 0 || y < 0 || x > max_pos || y > max_pos) {
            let x = x.try_into().unwrap();
            let y = y.try_into().unwrap();
            adjacent_pos.push([x, y]);
        }
    }

    adjacent_pos
}

fn a_star_2(risk_map: &Vec<Vec<usize>>) -> usize {
    let max_pos: i32 = (risk_map.len() * 5 - 1).try_into().unwrap();

    let mut open: HashMap<[usize; 2], Cell> = HashMap::new();
    let mut closed: HashMap<[usize; 2], Cell> = HashMap::new();
    open.insert([0, 0], Cell { f: 0, g: 0, h: 0 });

    let mut total_risk = 0;
    while !open.is_empty() {
        let (min_cell_key, _) = open.iter().min_by_key(|(_, value)| value.f).unwrap();
        let min_cell_key = min_cell_key.clone();
        let min_cell = open.remove(&min_cell_key).unwrap();
        closed.insert(min_cell_key, min_cell);

        let adjacent_cells = get_adjacent_cells(&min_cell_key, max_pos);
        let max_pos: usize = max_pos.try_into().unwrap();
        if adjacent_cells.contains(&[max_pos, max_pos]) {
            total_risk = min_cell.g;
            break;
        }
        let adjacent_cells = adjacent_cells
            .iter()
            .filter(|cell_pos| !closed.contains_key(*cell_pos))
            .map(|cell_pos| {
                let x = cell_pos[0];
                let y = cell_pos[1];
                let g = min_cell.g + wraped_risk(x, y, risk_map);
                let h = x.abs_diff(min_cell_key[0]) + y.abs_diff(min_cell_key[1]);
                let f = g + h;
                (cell_pos, Cell { f, g, h })
            });
        adjacent_cells.for_each(|(cell_pos, cell)| {
            open.entry(*cell_pos)
                .and_modify(|x| {
                    if x.f > cell.f {
                        x.f = cell.f;
                        x.g = cell.g;
                        x.h = cell.h;
                    }
                })
                .or_insert(cell);
        });
    }

    let max_pos: usize = max_pos.try_into().unwrap();
    total_risk + wraped_risk(max_pos, max_pos, risk_map)
}

fn wraped_risk(x: usize, y: usize, risk_map: &Vec<Vec<usize>>) -> usize {
    let max_pos = risk_map.len();
    let w_x = x % max_pos;
    let w_y = y % max_pos;

    let risk = risk_map[w_y][w_x] - 1 + (x / max_pos) + (y / max_pos);

    risk % 9 + 1
}

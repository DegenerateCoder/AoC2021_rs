pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut file = file.lines();

    let crab_pos = file.next().unwrap().split(',');
    let crab_pos = crab_pos.map(|x| x.parse::<u32>().unwrap());
    let crab_pos: Vec<u32> = crab_pos.collect();

    let max_pos = crab_pos.iter().max().unwrap();
    let mut fuel_costs: Vec<u32> = vec![0; (max_pos + 1) as usize];

    for (pos, fuel_cost) in fuel_costs.iter_mut().enumerate() {
        let mut sum_cost = 0;
        crab_pos
            .iter()
            .for_each(|x| sum_cost += x.abs_diff(pos as u32));
        *fuel_cost = sum_cost;
    }

    let min_fuel_cost = fuel_costs.iter().min().unwrap();

    *min_fuel_cost
}

pub fn part02(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut file = file.lines();

    let crab_pos = file.next().unwrap().split(',');
    let crab_pos = crab_pos.map(|x| x.parse::<u32>().unwrap());
    let crab_pos: Vec<u32> = crab_pos.collect();

    let max_pos = crab_pos.iter().max().unwrap();
    let mut fuel_costs: Vec<u32> = vec![0; (max_pos + 1) as usize];

    for (pos, fuel_cost) in fuel_costs.iter_mut().enumerate() {
        let mut sum_cost = 0;
        crab_pos
            .iter()
            .for_each(|x| sum_cost += calculate_fuel_cost(x.abs_diff(pos as u32)));
        *fuel_cost = sum_cost;
    }

    let min_fuel_cost = fuel_costs.iter().min().unwrap();

    *min_fuel_cost
}

// sum of an arithmetic series
// https://en.wikipedia.org/wiki/Arithmetic_progression#Sum
pub fn calculate_fuel_cost(pos: u32) -> u32 {
    let n = pos;
    let a1 = 1;
    let an = pos;
    let fuel_cost = (n / 2) * (a1 + an);

    fuel_cost
}

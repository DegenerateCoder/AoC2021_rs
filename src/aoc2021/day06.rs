pub fn part01(file_path: &str) -> u64 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut file = file.lines();

    let ages_list = file.next().unwrap().split(',');
    let ages_list = ages_list.map(|x| x.parse::<u32>().unwrap());
    let ages_list: Vec<u32> = ages_list.collect();

    let num = simulate(ages_list, 80);

    num
}

pub fn part02(file_path: &str) -> u64 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut file = file.lines();

    let ages_list = file.next().unwrap().split(',');
    let ages_list = ages_list.map(|x| x.parse::<u32>().unwrap());
    let ages_list: Vec<u32> = ages_list.collect();

    let num = simulate(ages_list, 256);

    num
}

const DAYS_MAX: usize = 8;
fn simulate(ages_list: Vec<u32>, day_num: u32) -> u64 {
    let mut lanternfish_in_day: [u64; DAYS_MAX + 1] = [0; DAYS_MAX + 1];

    ages_list
        .iter()
        .for_each(|x| lanternfish_in_day[*x as usize] += 1);

    for _ in 0..day_num {
        let mut new_lanternfish_in_day: [u64; DAYS_MAX + 1] = [0; DAYS_MAX + 1];
        for (i, lanternfish_num) in lanternfish_in_day.iter().enumerate() {
            if i == 0 {
                new_lanternfish_in_day[DAYS_MAX] = *lanternfish_num;
                new_lanternfish_in_day[6] = *lanternfish_num;
            } else {
                new_lanternfish_in_day[i - 1] += *lanternfish_num;
            }
        }
        lanternfish_in_day = new_lanternfish_in_day;
    }

    lanternfish_in_day.iter().sum()
}

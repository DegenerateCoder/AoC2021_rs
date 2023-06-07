use std::collections::HashMap;

struct PairInsertionRule<'a> {
    pair: &'a str,
    insert: char,
}

pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut file = file.lines();

    let polymer_template = file.next().unwrap().trim();
    // skip empty line
    file.next();

    let file = file.map(|line| {
        let mut line = line.split(" -> ");
        PairInsertionRule {
            pair: line.next().unwrap(),
            insert: line.next().unwrap().chars().next().unwrap(),
        }
    });
    let mut rules: HashMap<[char; 2], char> = HashMap::new();
    file.for_each(|rule| {
        let pair: Vec<char> = rule.pair.chars().collect();
        rules.insert([pair[0], pair[1]], rule.insert);
    });

    let mut polymer: Vec<char> = polymer_template.chars().collect();
    for _ in 1..=10 {
        polymer = step_v1(&polymer, &rules);
    }

    let mut char_count: HashMap<char, usize> = HashMap::new();
    polymer.iter().for_each(|c| {
        char_count
            .entry(*c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    let max = char_count.iter().max_by_key(|x| x.1).unwrap().1;
    let min = char_count.iter().min_by_key(|x| x.1).unwrap().1;

    let answer = max - min;

    answer as u32
}

pub fn part02_v1(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut file = file.lines();

    let polymer_template = file.next().unwrap().trim();
    // skip empty line
    file.next();

    let file = file.map(|line| {
        let mut line = line.split(" -> ");
        PairInsertionRule {
            pair: line.next().unwrap(),
            insert: line.next().unwrap().chars().next().unwrap(),
        }
    });
    let mut rules: HashMap<[char; 2], char> = HashMap::new();
    file.for_each(|rule| {
        let pair: Vec<char> = rule.pair.chars().collect();
        rules.insert([pair[0], pair[1]], rule.insert);
    });

    let mut polymer: Vec<char> = polymer_template.chars().collect();
    for _ in 1..=40 {
        polymer = step_v1(&polymer, &rules);
    }

    let mut char_count: HashMap<char, usize> = HashMap::new();
    polymer.iter().for_each(|c| {
        char_count
            .entry(*c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    let max = char_count.iter().max_by_key(|x| x.1).unwrap().1;
    let min = char_count.iter().min_by_key(|x| x.1).unwrap().1;

    let answer = max - min;

    answer as u32
}

pub fn part02_v2(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut file = file.lines();

    let polymer_template = file.next().unwrap().trim();
    // skip empty line
    file.next();

    let file = file.map(|line| {
        let mut line = line.split(" -> ");
        PairInsertionRule {
            pair: line.next().unwrap(),
            insert: line.next().unwrap().chars().next().unwrap(),
        }
    });
    let mut rules: HashMap<[char; 2], char> = HashMap::new();
    file.for_each(|rule| {
        let pair: Vec<char> = rule.pair.chars().collect();
        rules.insert([pair[0], pair[1]], rule.insert);
    });

    let mut polymer_pairs: HashMap<[char; 2], usize> = HashMap::new();
    let polymer_template: Vec<char> = polymer_template.chars().collect();
    let mut pair = [' ', ' '];
    pair[0] = polymer_template[0];
    pair[1] = polymer_template[0];
    for element in polymer_template.iter().skip(1) {
        pair[0] = pair[1];
        pair[1] = *element;
        polymer_pairs
            .entry(pair)
            .and_modify(|pair| *pair += 1)
            .or_insert(1);
    }

    for _ in 1..=40 {
        polymer_pairs = step_v2(polymer_pairs, &rules);
    }

    let mut char_count: HashMap<char, usize> = HashMap::new();
    char_count.insert(polymer_template[0], 1);
    polymer_pairs.iter().for_each(|(pair, p_count)| {
        char_count
            .entry(pair[1])
            .and_modify(|count| *count += p_count)
            .or_insert(*p_count);
    });

    let max = char_count.iter().max_by_key(|x| x.1).unwrap().1;
    let min = char_count.iter().min_by_key(|x| x.1).unwrap().1;

    let answer = max - min;

    answer
}

fn step_v1(polymer_template: &Vec<char>, rules: &HashMap<[char; 2], char>) -> Vec<char> {
    let mut polymer: Vec<char> = Vec::new();
    let mut pair = [' ', ' '];
    pair[0] = polymer_template[0];
    pair[1] = polymer_template[0];
    polymer.push(polymer_template[0]);

    let polymer_template = polymer_template.iter().skip(1);
    for element in polymer_template {
        pair[0] = pair[1];
        pair[1] = *element;
        if let Some(rule) = rules.get(&pair) {
            polymer.push(*rule);
        }
        polymer.push(*element);
    }

    polymer
}

fn step_v2(
    polymer_pairs: HashMap<[char; 2], usize>,
    rules: &HashMap<[char; 2], char>,
) -> HashMap<[char; 2], usize> {
    let mut new_polymer_pairs: HashMap<[char; 2], usize> = HashMap::new();
    polymer_pairs.iter().for_each(|(pair, p_count)| {
        if let Some(rule) = rules.get(pair) {
            let new_left_pair = [pair[0], *rule];
            new_polymer_pairs
                .entry(new_left_pair)
                .and_modify(|count| *count += p_count)
                .or_insert(*p_count);

            let new_right_pair = [*rule, pair[1]];
            new_polymer_pairs
                .entry(new_right_pair)
                .and_modify(|count| *count += p_count)
                .or_insert(*p_count);
        }
    });

    new_polymer_pairs
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

struct SmallCave {
    id: String,
    connections: Vec<Weak<RefCell<Cave>>>,
}

struct LargeCave {
    id: String,
    connections: Vec<Weak<RefCell<Cave>>>,
}

enum Cave {
    SmallCave(SmallCave),
    LargeCave(LargeCave),
}

pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let nav_map = file.map(|x| x.split('-'));
    let mut caves: HashMap<String, Rc<RefCell<Cave>>> = HashMap::new();
    for mut nav in nav_map {
        let from = nav.next().unwrap();
        let to = nav.next().unwrap();
        if !caves.contains_key(from) {
            insert_cave(from, &mut caves);
        }
        if !caves.contains_key(to) {
            insert_cave(to, &mut caves);
        }
        let from_cave = Rc::clone(caves.get(from).unwrap());
        let mut from_cave = from_cave.borrow_mut();
        let from_cave = from_cave.deref_mut();
        let to_cave = Rc::downgrade(caves.get(to).unwrap());
        match from_cave {
            Cave::LargeCave(cave) => cave.connections.push(to_cave),
            Cave::SmallCave(cave) => cave.connections.push(to_cave),
        }
        // Revers connection
        let from_cave = Rc::clone(caves.get(to).unwrap());
        let mut from_cave = from_cave.borrow_mut();
        let from_cave = from_cave.deref_mut();
        let to_cave = Rc::downgrade(caves.get(from).unwrap());
        match from_cave {
            Cave::LargeCave(cave) => cave.connections.push(to_cave),
            Cave::SmallCave(cave) => cave.connections.push(to_cave),
        }
    }

    let mut paths: Vec<String> = Vec::new();
    travers_cave_system(&caves, &mut paths, "start".to_string(), false);

    paths.len() as u32
}

pub fn part02_v2(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let nav_map = file.map(|x| x.split('-'));
    let mut caves: HashMap<String, Rc<RefCell<Cave>>> = HashMap::new();
    for mut nav in nav_map {
        let from = nav.next().unwrap();
        let to = nav.next().unwrap();
        if !caves.contains_key(from) {
            insert_cave(from, &mut caves);
        }
        if !caves.contains_key(to) {
            insert_cave(to, &mut caves);
        }
        let from_cave = Rc::clone(caves.get(from).unwrap());
        let mut from_cave = from_cave.borrow_mut();
        let from_cave = from_cave.deref_mut();
        let to_cave = Rc::downgrade(caves.get(to).unwrap());
        match from_cave {
            Cave::LargeCave(cave) => cave.connections.push(to_cave),
            Cave::SmallCave(cave) => cave.connections.push(to_cave),
        }
        // Revers connection
        let from_cave = Rc::clone(caves.get(to).unwrap());
        let mut from_cave = from_cave.borrow_mut();
        let from_cave = from_cave.deref_mut();
        let to_cave = Rc::downgrade(caves.get(from).unwrap());
        match from_cave {
            Cave::LargeCave(cave) => cave.connections.push(to_cave),
            Cave::SmallCave(cave) => cave.connections.push(to_cave),
        }
    }

    travers_cave_system_v2(&caves, "start", true, vec!["start"])
}

fn insert_cave(id: &str, caves: &mut HashMap<String, Rc<RefCell<Cave>>>) {
    let is_large_cave = id.chars().next().unwrap().is_uppercase();
    let id = id.to_string();
    if is_large_cave {
        let cave = LargeCave {
            id: id.clone(),
            connections: Vec::new(),
        };
        let cave = Rc::new(RefCell::new(Cave::LargeCave(cave)));
        caves.insert(id, cave);
    } else {
        let cave = SmallCave {
            id: id.clone(),
            connections: Vec::new(),
        };
        let cave = Rc::new(RefCell::new(Cave::SmallCave(cave)));
        caves.insert(id, cave);
    }
}

fn travers_cave_system_v2(
    caves: &HashMap<String, Rc<RefCell<Cave>>>,
    current_cave: &str,
    visit_twice: bool,
    visited_small_cave: Vec<&str>,
) -> usize {
    let current_cave = caves.get(current_cave).unwrap();
    let current_cave = current_cave.borrow();
    let current_cave = current_cave.deref();

    let mut path_num = 0;
    match current_cave {
        Cave::LargeCave(cave) => {
            for cave in cave.connections.iter() {
                let cave = cave.upgrade().unwrap();
                let cave = cave.borrow();
                let cave = cave.deref();
                path_num += add_cave_to_path_and_traverse_v2(
                    cave,
                    caves,
                    visit_twice,
                    visited_small_cave.clone(),
                );
            }
        }
        Cave::SmallCave(cave) => {
            for cave in cave.connections.iter() {
                let cave = cave.upgrade().unwrap();
                let cave = cave.borrow();
                let cave = cave.deref();
                path_num += add_cave_to_path_and_traverse_v2(
                    cave,
                    caves,
                    visit_twice,
                    visited_small_cave.clone(),
                );
            }
        }
    }

    path_num
}

fn add_cave_to_path_and_traverse_v2<'a>(
    cave: &'a Cave,
    caves: &HashMap<String, Rc<RefCell<Cave>>>,
    mut visit_twice: bool,
    mut visited_small_caves: Vec<&'a str>,
) -> usize {
    match cave {
        Cave::LargeCave(cave) => {
            travers_cave_system_v2(caves, &cave.id, visit_twice, visited_small_caves)
        }

        Cave::SmallCave(cave) => {
            if cave.id == "start" {
                return 0;
            }
            let mut visited_count = 0;
            visited_small_caves.iter().for_each(|x| {
                if **x == cave.id {
                    visited_count += 1
                }
            });
            if visited_count < 1 || (visited_count < 2 && visit_twice) {
                if cave.id == "end" {
                    return 1;
                } else {
                    visited_small_caves.push(&cave.id);
                    if visited_count == 1 {
                        visit_twice = false;
                    }
                    travers_cave_system_v2(caves, &cave.id, visit_twice, visited_small_caves)
                }
            } else {
                return 0;
            }
        }
    }
}

fn travers_cave_system(
    caves: &HashMap<String, Rc<RefCell<Cave>>>,
    paths: &mut Vec<String>,
    current_path: String,
    visit_twice: bool,
) {
    let current_cave = current_path.split(',').last().unwrap();
    let current_cave = caves.get(current_cave).unwrap();
    let current_cave = current_cave.borrow();
    let current_cave = current_cave.deref();

    match current_cave {
        Cave::LargeCave(cave) => {
            for cave in cave.connections.iter() {
                let cave = cave.upgrade().unwrap();
                let cave = cave.borrow();
                let cave = cave.deref();
                add_cave_to_path_and_traverse(cave, &current_path, caves, paths, visit_twice);
            }
        }
        Cave::SmallCave(cave) => {
            for cave in cave.connections.iter() {
                let cave = cave.upgrade().unwrap();
                let cave = cave.borrow();
                let cave = cave.deref();
                add_cave_to_path_and_traverse(cave, &current_path, caves, paths, visit_twice);
            }
        }
    }
}

fn add_cave_to_path_and_traverse(
    cave: &Cave,
    current_path: &String,
    caves: &HashMap<String, Rc<RefCell<Cave>>>,
    paths: &mut Vec<String>,
    visit_twice: bool,
) {
    let visited_small_caves = current_path.clone();
    let visited_small_caves = visited_small_caves.split(',').filter(|x| {
        let is_large_cave = x.chars().next().unwrap().is_uppercase();
        !is_large_cave
    });
    match cave {
        Cave::LargeCave(cave) => {
            let mut current_path = current_path.clone();
            current_path.push_str(",");
            current_path.push_str(&cave.id);
            travers_cave_system(caves, paths, current_path, visit_twice);
        }

        Cave::SmallCave(cave) => {
            let visited_count = visited_small_caves
                .clone()
                .filter(|x| x.contains(&cave.id))
                .count();

            let mut visited = visited_count > 0;
            if visit_twice && !current_path.starts_with("[2x]") {
                visited = visited_count > 1;
            }
            if !visited {
                let mut current_path = current_path.clone();
                current_path.push_str(",");
                current_path.push_str(&cave.id);
                if cave.id == "end" {
                    paths.push(current_path);
                } else if cave.id != "start" {
                    if visited_count == 0 {
                        travers_cave_system(caves, paths, current_path, visit_twice);
                    } else {
                        let current_path = current_path.replace("start", "[2x]start");
                        travers_cave_system(caves, paths, current_path, visit_twice);
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn part02_v1(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let nav_map = file.map(|x| x.split('-'));
    let mut caves: HashMap<String, Rc<RefCell<Cave>>> = HashMap::new();
    for mut nav in nav_map {
        let from = nav.next().unwrap();
        let to = nav.next().unwrap();
        if !caves.contains_key(from) {
            insert_cave(from, &mut caves);
        }
        if !caves.contains_key(to) {
            insert_cave(to, &mut caves);
        }
        let from_cave = Rc::clone(caves.get(from).unwrap());
        let mut from_cave = from_cave.borrow_mut();
        let from_cave = from_cave.deref_mut();
        let to_cave = Rc::downgrade(caves.get(to).unwrap());
        match from_cave {
            Cave::LargeCave(cave) => cave.connections.push(to_cave),
            Cave::SmallCave(cave) => cave.connections.push(to_cave),
        }
        // Revers connection
        let from_cave = Rc::clone(caves.get(to).unwrap());
        let mut from_cave = from_cave.borrow_mut();
        let from_cave = from_cave.deref_mut();
        let to_cave = Rc::downgrade(caves.get(from).unwrap());
        match from_cave {
            Cave::LargeCave(cave) => cave.connections.push(to_cave),
            Cave::SmallCave(cave) => cave.connections.push(to_cave),
        }
    }

    let mut paths: Vec<String> = Vec::new();
    travers_cave_system(&caves, &mut paths, "start".to_string(), true);

    paths.len() as u32
}

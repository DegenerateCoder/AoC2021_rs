mod aoc2021;

fn main() {
    println!("Test data:");

    println!("\tDay 01: ");
    let day01_part1 = aoc2021::day01::part1("./puzzle_input/day01_test.txt");
    println!("\t\tpart 1: {day01_part1}");
    let day01_part2 = aoc2021::day01::part2("./puzzle_input/day01_test.txt");
    let day01_part2_v2 = aoc2021::day01::part2_v2("./puzzle_input/day01_test.txt");
    println!("\t\tpart 2: v1: {day01_part2}; v2: {day01_part2_v2}");

    println!("\tDay 02: ");
    let day02_part1 = aoc2021::day02::part01("./puzzle_input/day02_test.txt");
    println!("\t\tpart 1: {day02_part1}");
    let day02_part2 = aoc2021::day02::part02("./puzzle_input/day02_test.txt");
    println!("\t\tpart 2: {day02_part2}");

    println!("\tDay 03: ");
    let day03_part1 = aoc2021::day03::part01("./puzzle_input/day03_test.txt");
    println!("\t\tpart 1: {day03_part1}");

    println!("Puzzle data:");

    println!("\tDay 01: ");
    let day01_part1 = aoc2021::day01::part1("./puzzle_input/day01.txt");
    println!("\t\tpart 1: {day01_part1}");
    let day01_part2 = aoc2021::day01::part2("./puzzle_input/day01.txt");
    let day01_part2_v2 = aoc2021::day01::part2_v2("./puzzle_input/day01.txt");
    println!("\t\tpart 2: v1: {day01_part2}; v2: {day01_part2_v2}");

    println!("\tDay 02: ");
    let day02_part1 = aoc2021::day02::part01("./puzzle_input/day02.txt");
    println!("\t\tpart 1: {day02_part1}");
    let day02_part2 = aoc2021::day02::part02("./puzzle_input/day02.txt");
    println!("\t\tpart 2: {day02_part2}");

    println!("\tDay 03: ");
    let day03_part1 = aoc2021::day03::part01("./puzzle_input/day03.txt");
    println!("\t\tpart 1: {day03_part1}");
}

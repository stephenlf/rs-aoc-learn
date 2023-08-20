mod forest;

fn main() {
    let mut forest = forest::Forest::new("../inputs/day_8.txt");
    forest.calc_visibility();
    println!("Part 1: {}", forest.sum_visibility());

    let mut forest = forest::Forest::new("../inputs/day_8.txt");
    forest.calc_multi();
    println!("Part 1 (multithreaded): {}", forest.sum_visibility());
}
mod forest;

fn main() {
    let mut forest = forest::Forest::new("../inputs/day_8.txt");
    println!("Part 1: {}", forest.solve_single());
}
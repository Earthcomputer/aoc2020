use crate::util;

pub fn run_easy() {
    let maze = read_maze();
    let num_trees = get_number_of_trees(1, 3, &maze);
    println!("{}", num_trees);
}

pub fn run_hard() {
    let maze = read_maze();
    let mut product = 1;
    product *= get_number_of_trees(1, 1, &maze);
    product *= get_number_of_trees(1, 3, &maze);
    product *= get_number_of_trees(1, 5, &maze);
    product *= get_number_of_trees(1, 7, &maze);
    product *= get_number_of_trees(2, 1, &maze);
    println!("{}", product);
}

fn get_number_of_trees(dy: usize, dx: usize, maze: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    let mut x = 0;
    for y in (0..maze.len()).step_by(dy) {
        if maze[y][x % maze[y].len()] {
            count += 1;
        }
        x += dx;
    }
    return count;
}

fn read_maze() -> Vec<Vec<bool>> {
    return util::get_input_lines().map(|line| line.chars().map(|c| c == '#').collect()).collect()
}

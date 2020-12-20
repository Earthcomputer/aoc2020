use crate::util;

type Tile = Vec<Vec<bool>>;

pub fn run_easy() {
    let input = get_input();
    let tiles = get_input_combinations(input);
    let grid = solve_jigsaw(&tiles);
    let product = grid[0][0].0 as u64
        * grid[0].last().unwrap().0 as u64
        * grid.last().unwrap()[0].0 as u64
        * grid.last().unwrap().last().unwrap().0 as u64;
    println!("{}", product);
}

pub fn run_hard() {
    let input = get_input();
    let tiles = get_input_combinations(input);
    let grid = solve_jigsaw(&tiles);

    let image: Vec<Vec<_>> = grid.iter()
        .flat_map(|grid_row| (1..grid_row[0].1.len()-1)
            .map(move |y| grid_row.iter()
                .flat_map(|(_, grid_tile)| grid_tile[y][1..grid_tile[0].len()-1].iter()
                    .map(|b| b.clone()))
                .collect()))
        .collect();

    let sea_monster: Tile = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   "
    ].iter()
        .map(|s| s.chars().map(|c| c == '#').collect())
        .collect();
    let sea_monsters = get_rotations(&sea_monster);

    let mut is_sea_monster: Vec<_> = image.iter().map(|_| vec![false; image[0].len()]).collect();
    for sea_monster in &sea_monsters {
        for y in 0..image.len()-sea_monster.len() {
            for x in 0..image[0].len()-sea_monster[0].len() {
                let mut found_sea_monster = true;

                'sea_monster_search: for dy in 0..sea_monster.len() {
                    for dx in 0..sea_monster[0].len() {
                        if sea_monster[dy][dx] && !image[y + dy][x + dx] {
                            found_sea_monster = false;
                            break 'sea_monster_search;
                        }
                    }
                }

                if found_sea_monster {
                    for dy in 0..sea_monster.len() {
                        for dx in 0..sea_monster[0].len() {
                            if sea_monster[dy][dx] {
                                is_sea_monster[y + dy][x + dx] = true;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut count = 0;
    for y in 0..image.len() {
        for x in 0..image[0].len() {
            if image[y][x] && !is_sea_monster[y][x] {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn get_input_combinations(input: Vec<(i32, Tile)>) -> Vec<(i32, [Tile; 8])> {
    input.iter()
        .map(|(id, tile)| (*id, get_rotations(tile)))
        .collect()
}

fn solve_jigsaw(tiles: &Vec<(i32, [Tile; 8])>) -> Vec<Vec<(i32, &Tile)>> {
    fn tile_refs(tiles: &[Tile; 8]) -> [&Tile; 8] {
        [&tiles[0], &tiles[1], &tiles[2], &tiles[3], &tiles[4], &tiles[5], &tiles[6], &tiles[7]]
    }
    let tiles: Vec<_> = tiles.iter()
        .map(|(id, tiles)| (*id, tile_refs(tiles)))
        .collect();
    for width in 1..=(tiles.len() as f32).sqrt().ceil() as usize {
        if tiles.len() % width == 0 {
            let mut grid: Vec<_> = (0..tiles.len() / width).map(|_| vec![(0, tiles[0].1[0]); width]).collect();
            if solve_jigsaw_at_width(&mut grid, &tiles, 0, 0, width) {
                return grid;
            }
        }
    }
    unreachable!();
}

fn solve_jigsaw_at_width<'a>(grid: &mut Vec<Vec<(i32, &'a Tile)>>, remaining_tiles: &Vec<(i32, [&'a Tile; 8])>, x: usize, y: usize, width: usize) -> bool {
    let (next_x, next_y) = if x == width - 1 {
        (0, y + 1)
    } else {
        (x + 1, y)
    };

    for (i, (id, tiles)) in remaining_tiles.iter().enumerate() {
        for tile in tiles {
            let mut valid = true;
            if x != 0 && !is_adjacent_horizontal(&grid[y][x - 1].1, tile) {
                valid = false;
            } else if y != 0 && !is_adjacent_vertical(&grid[y - 1][x].1, tile) {
                valid = false;
            }
            if valid {
                let mut new_remaining_tiles = remaining_tiles.clone();
                new_remaining_tiles.remove(i);
                grid[y][x] = (*id, tile);
                if new_remaining_tiles.is_empty() {
                    return true;
                }
                if solve_jigsaw_at_width(grid, &new_remaining_tiles, next_x, next_y, width) {
                    return true;
                }
            }
        }
    }

    return false;
}

fn rotate_tile(tile: &Tile) -> Tile {
    let mut new_tile = Vec::new();
    for x in 0..tile[0].len() {
        new_tile.push(vec![false; tile.len()]);
        for y in 0..tile.len() {
            new_tile[x][tile.len() - 1 - y] = tile[y][x];
        }
    }
    return new_tile;
}

fn get_rotations(tile: &Tile) -> [Tile; 8] {
    let first = rotate_tile(tile);
    let second = rotate_tile(&first);
    let third = rotate_tile(&second);
    let mut fourth = tile.clone();
    fourth.reverse();
    let fifth = rotate_tile(&fourth);
    let sixth = rotate_tile(&fifth);
    let seventh = rotate_tile(&sixth);
    return [tile.clone(), first, second, third, fourth, fifth, sixth, seventh];
}

fn is_adjacent_horizontal(left: &Tile, right: &Tile) -> bool {
    if left.len() != right.len() {
        return false;
    }
    for y in 0..left.len() {
        if *left[y].last().unwrap() != right[y][0] {
            return false;
        }
    }
    return true;
}

fn is_adjacent_vertical(top: &Tile, bottom: &Tile) -> bool {
    if top[0].len() != bottom[0].len() {
        return false;
    }
    for x in 0..top[0].len() {
        if top.last().unwrap()[x] != bottom[0][x] {
            return false;
        }
    }
    return true;
}

fn get_input() -> Vec<(i32, Tile)> {
    let lines: Vec<_> = util::get_input_lines().collect();
    return lines.as_slice()
        .split(|s| s.is_empty())
        .map(|lines| {
            let tile_id = lines[0].strip_prefix("Tile ").unwrap()
                .strip_suffix(":").unwrap()
                .parse().unwrap();
            let tile = lines.iter()
                .skip(1)
                .map(|s| s.chars().map(|c| c == '#').collect())
                .collect();
            (tile_id, tile)
        })
        .collect();
}

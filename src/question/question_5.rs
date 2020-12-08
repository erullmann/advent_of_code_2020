// # Question 5 Toboggan Trajectory

const SLOPE: Slope = Slope {run: 3, rise: 1};

pub fn answer() -> super::Answer {
    let file = super::get_file("data/day_3/input.txt");
    match file {
        Err(_) => super::Answer{ result: file, question: 3 },
        Ok(contents) => {
            let tree_count = count_trees(&contents, SLOPE);
            super::Answer{ result: Ok(tree_count.to_string()), question: 5 }
        }
    }
}

pub struct Slope {
    pub run: usize,
    pub rise: usize
}

pub fn count_trees(contents: &String, slope: Slope) -> usize {
    let slope_width = contents.lines().next().unwrap().len();
    println!("slope width {}", slope_width);
    let mut horizontal_index = 0;
    let mut tree_count = 0;

    for (vertical_index, line) in contents.lines().enumerate() {
        if (vertical_index % slope.rise) == 0 {
            let terrain: char  = line.get(0..horizontal_index+1).unwrap().chars().last().unwrap();

            println!("terrain {}, {}, {}", terrain, vertical_index, horizontal_index);

            if terrain == '#' {
                tree_count += 1
            }
            if terrain == '\n' {
                println!("newline!")
            }
            horizontal_index = advance_slope(horizontal_index, &slope, slope_width);
        }
    }
    return tree_count;
}

fn advance_slope(index: usize, slope: &Slope, slope_width: usize) -> usize {
    (index + slope.run) % slope_width
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_slope(){
        let terrain: String = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#".to_string();
        assert_eq!(7, count_trees(&terrain, SLOPE));
        assert_eq!(2, count_trees(&terrain, Slope {run: 1, rise: 1}));
        assert_eq!(4, count_trees(&terrain, Slope {run: 7, rise: 1}));
        assert_eq!(2, count_trees(&terrain, Slope {run: 1, rise: 2}));
    }

    #[test]
    fn find_long_slope(){
        let terrain: String = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
#.##...#...
#...##....#
.#..#...#.#".to_string();
        assert_eq!(5, count_trees(&terrain, Slope {run: 1, rise: 2}));
    }
}
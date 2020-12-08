use super::question_5::count_trees;
use super::question_5::Slope;

// # Question 6 Toboggan Trajectory

pub fn answer() -> super::Answer {
    let file = super::get_file("data/day_3/input.txt");
    match file {
        Err(_) => super::Answer{ result: file, question: 6 },
        Ok(contents) => super::Answer{ result: Ok(consider_all_slopes(contents).to_string()), question: 6 }
    }
}

fn consider_all_slopes(contents: String) -> usize {
    let count_1_1 = count_trees(&contents, Slope{ run: 1, rise: 1});
    let count_3_1 = count_trees(&contents, Slope{ run: 3, rise: 1});
    let count_5_1 = count_trees(&contents, Slope{ run: 5, rise: 1});
    let count_7_1 = count_trees(&contents, Slope{ run: 7, rise: 1});
    let count_1_2 = count_trees(&contents, Slope{ run: 1, rise: 2});

    return count_1_1 * count_3_1 * count_5_1 * count_7_1 * count_1_2
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
        assert_eq!(336, consider_all_slopes(terrain));
    }
}

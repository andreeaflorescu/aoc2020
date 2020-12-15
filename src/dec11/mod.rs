use std::collections::HashMap;

fn push_in_turn(value: u64, turn: usize, turns: &mut HashMap<u64, Vec<usize>>) {
    if let Some(turns_for_elem) = turns.get_mut(&value) {
        if turns_for_elem.len() == 1 {
            turns_for_elem.push(turn);
        } else {
            turns_for_elem.remove(0);
            turns_for_elem.push(turn);
        }
    } else {
        turns.insert(value, vec![turn]);
    }
}

fn the_xth_spoken_number(input: Vec<u64>, x: usize) -> u64 {
    let mut turns = HashMap::<u64, Vec<usize>>::new();
    let mut last_spoken_value = 0u64;
    for (turn, nr) in input.iter().enumerate() {
        turns.insert(*nr, vec![turn + 1]);
        last_spoken_value = *nr;
    }

    let current_turn = input.len() + 1;

    for turn in current_turn..(x + 1) {
        if let Some(turns_for_elem) = turns.get_mut(&last_spoken_value) {
            if turns_for_elem.len() == 1 {
                last_spoken_value = 0;
            } else {
                last_spoken_value = (turns_for_elem[1] - turns_for_elem[0]) as u64;
            }
        }
        push_in_turn(last_spoken_value, turn, &mut turns);
    }

    last_spoken_value
}

pub fn solve_puzzle(input: Vec<u64>) {
    println!(
        "the 2020th spoken number is: {}",
        the_xth_spoken_number(input.clone(), 2020)
    );
    println!(
        "the 30000000 spoken number is: {}",
        the_xth_spoken_number(input, 30000000)
    );
}

#[cfg(test)]
mod tests {
    use crate::dec11::{solve_puzzle, the_xth_spoken_number};

    #[test]
    fn simple_test() {
        let input = vec![0, 3, 6];
        assert_eq!(the_xth_spoken_number(input, 10), 0);
    }

    #[test]
    fn test_part_1() {
        let input = vec![1, 3, 2];
        assert_eq!(the_xth_spoken_number(input, 2020), 1);

        let input = vec![2, 1, 3];
        assert_eq!(the_xth_spoken_number(input, 2020), 10);

        let input = vec![1, 2, 3];
        assert_eq!(the_xth_spoken_number(input, 2020), 27);

        let input = vec![2, 3, 1];
        assert_eq!(the_xth_spoken_number(input, 2020), 78);

        let input = vec![3, 2, 1];
        assert_eq!(the_xth_spoken_number(input, 2020), 438);

        let input = vec![3, 1, 2];
        assert_eq!(the_xth_spoken_number(input, 2020), 1836);
    }

    #[test]
    fn run_solve_puzzle() {
        let input = vec![1, 0, 18, 10, 19, 6];
        solve_puzzle(input);
    }
}

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        NQueen::solve_n_queens(n)
    }
}

pub struct NQueen {
    pub results: Vec<Vec<Position>>,
    pub dimension: i32,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Position (pub i32, pub i32);

impl NQueen {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut n_queen = NQueen {
            results: Vec::new(),
            dimension: n
        };

        n_queen.do_solution(0, &mut Vec::new());
        return n_queen.marshal();
    }

    fn marshal(&self) -> Vec<Vec<String>> {
        let mut res = Vec::new();
        for group in self.results.iter() {
            let mut sub_res : Vec<String> = Vec::new();
            for pos in group.iter() {
                sub_res.push(self.convert(pos));
            }
            res.push(sub_res.clone());
        }
        return res;
    }

    fn convert(&self, pos: &Position) -> String {
        let mut res: String = String::from("");
        for i in 0..self.dimension {
            if i == pos.1 {
                res.push('Q');
            } else {
                res.push('.');
            }
        }
        return res;
    }

    fn do_solution(&mut self, row: i32, track: &mut Vec<Position>) {

        if row == self.dimension {
            self.results.push(track.clone());
            return;
        }

        for col in 0..self.dimension {
            let cur_pos = Position(row, col);

            if !self.is_valid(&cur_pos, track) { continue };

            // only if cur_pos is valid
            track.push(cur_pos);

            self.do_solution(row + 1, track);

            track.pop();

        }
    }

    fn get_diagonal_collision(&self, cur_pos: &Position) -> HashSet<Position> {
        let mut diagonal_collision: HashSet<Position> = HashSet::new();
        let mut delta = 1;

        // top left diagonal
        loop {
            if cur_pos.0 - delta < 0 || cur_pos.1 - delta < 0 {
                break;
            }
            diagonal_collision.insert(Position(
                cur_pos.0 - delta,
                cur_pos.1 - delta
            ));
            delta += 1;
        }

        // top right diagonal
        let mut delta = 1;
        loop {
            if cur_pos.0 - delta < 0 || cur_pos.1 + delta == self.dimension {
                break;
            }

            diagonal_collision.insert(Position(
                cur_pos.0 - delta,
                cur_pos.1 + delta
            ));
            delta += 1;
        }

        return diagonal_collision;
    }

    fn is_valid(&self, cur_pos: &Position, track: &Vec<Position>) -> bool {

        let diagonal_collision = self.get_diagonal_collision(cur_pos);

        for pos in track.iter() {
            // skip checking row because it is always different
            // col cannot be the same
            if pos.1 == cur_pos.1 {
                return false;
            }

            if diagonal_collision.contains(pos) {
                return false;
            }
        }

        return true;
    }
}

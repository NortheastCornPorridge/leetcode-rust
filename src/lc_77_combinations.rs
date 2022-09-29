struct Solution;
// copy below to LeetCode
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        Combine::combine(n, k)
    }
}

pub struct Combine {
    results: Vec<Vec<i32>>
}

impl Combine {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut c = Combine {
            results: Vec::new()
        };
        let mut track = Vec::new();
        c.do_solution(n, k, 1, &mut track);
        c.results
    }

    fn do_solution(&mut self, n: i32, k: i32, start_num: i32, track: &mut Vec<i32>) {
        for i in start_num..=n {
            track.push(i);

            if track.len() == k as usize {
                self.results.push(track.clone());
            }

            self.do_solution(n, k, i+1, track);

            track.pop();
        }
    }
}
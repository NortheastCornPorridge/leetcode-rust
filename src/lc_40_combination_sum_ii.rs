/*
Input: candidates = [10,1,2,7,6,1,5], target = 8
Output: 
[
    [1,1,6],
    [1,2,5],
    [1,7],
    [2,6]
]
*/
pub struct Solution;
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut track: Vec<i32> = Vec::new();
        let mut results: Vec<Vec<i32>> = Vec::new();
        candidates.sort(); 

        Solution::do_solution(&candidates, target, 0, &mut track, &mut results);

        return results;
    }

    fn do_solution(candidates: &[i32], target: i32, mut sum: i32, track: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
        if sum >= target {
            if sum == target {
                results.push(track.clone());
            }
            return;
        }

        for i in 0..candidates.len() {
            if i >= 1 && candidates[i] == candidates[i-1] {
                continue;
            }

            track.push(candidates[i]);
            sum += candidates[i];

            Solution::do_solution(&candidates[i+1..], target, sum, track, results);

            track.pop();
            sum -= candidates[i];
        }
    }
}
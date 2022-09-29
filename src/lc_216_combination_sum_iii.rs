pub struct Solution;
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut track: Vec<i32> = Vec::new();
        let mut results: Vec<Vec<i32>> = Vec::new();
        let nums = vec![1,2,3,4,5,6,7,8,9];
        
        Solution::do_solution(&nums, n, k, 0, &mut track, &mut results);

        results
    }
    
    fn do_solution(nums: &[i32], 
                   n: i32, 
                   k: i32, 
                   mut sum: i32, 
                   track: &mut Vec<i32>, 
                   results: &mut Vec<Vec<i32>>) 
    {
        if track.len() == k.try_into().unwrap()  {
            if sum == n {
                results.push(track.clone());
            }
            return;
        }

        for i in 0..nums.len() {
            track.push(nums[i]);
            sum += nums[i];

            Solution::do_solution(&nums[i+1..], n, k, sum, track, results);

            track.pop();
            sum -= nums[i];
        }
    }
}
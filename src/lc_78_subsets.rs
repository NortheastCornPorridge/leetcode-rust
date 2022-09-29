struct Solution;
// copy the line below to the leetcode
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        return Subsets::subsets(nums);
    }
}

pub struct Subsets {
    results: Vec<Vec<i32>>
}

impl Subsets {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut s = Subsets {
            results: Vec::new()
        };
        s.results.push(Vec::new()); // push empty set
        let mut track: Vec<i32> = Vec::new();
        s.do_solution(&nums[..], &mut track);
        return s.results;
    }

    pub fn do_solution(&mut self, sub_nums: &[i32],
                       track: &mut Vec<i32>)
    {
        for i in 0..sub_nums.len() {
            track.push(sub_nums[i]);
            self.results.push(track.clone());

            self.do_solution(&sub_nums[i+1..], track);

            track.pop();
        }
    }
}
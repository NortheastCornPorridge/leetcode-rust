struct Solution;
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        SubsetsII::subsets_with_dup(nums)
    }
}

pub struct SubsetsII {
    results: Vec<Vec<i32>>
}

impl SubsetsII {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut s = SubsetsII {
            results: Vec::new()
        };
        let mut t = vec![];
        nums.sort();
        s.results.push(t.clone());
        s.do_solution(&nums, &mut t);
        s.results
    }

    fn do_solution(&mut self, nums: &[i32], track: &mut Vec<i32>) {
        for i in 0..nums.len() {
            if i >= 1 && nums[i] == nums[i-1] {
                continue;
            }
            track.push(nums[i]);
            self.results.push(track.clone());
            self.do_solution(&nums[i+1..], track);
            track.pop();
        }
    }
}

// there might be another way: storing all the tracks into a set and use the values of each track to the input of the hash trait
// but it may increase some time complexity as each time we need to loop all elements to get the hash.


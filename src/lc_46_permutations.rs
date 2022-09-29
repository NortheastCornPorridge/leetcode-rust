#![allow(unused)]
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>>
{
    let mut results: Vec<Vec<i32>> = Vec::new();
    let mut used: Vec<bool> = vec![false; nums.len()];
    let mut track: Vec<i32> = Vec::new();

    do_permutation(&mut results,
                   &mut track,
                   &nums,
                   &mut used);

    results
}

fn do_permutation(results: &mut Vec<Vec<i32>>,
                  track: &mut Vec<i32>,
                  nums: & Vec<i32>,
                  used: &mut Vec<bool>)
{

    if track.len() == nums.len() {
        // here, we cannot use results.push(*track) as the track is a ref and it's shared in multiple places
        results.push(track.clone());
        return;
    }

    for (idx, it_n) in nums.iter().enumerate() {
        if used[idx] {
            continue;
        }
        track.push(*it_n);
        used[idx] = true;
        do_permutation(results, track, nums, used);
        track.pop();
        used[idx] = false;
    }

}

#[cfg(test)]
mod permute_tests {

    #[test]
    fn permute_works() {
        use super::*;

        let re = permute(vec![1,2,3]);

        dbg!(re);
    }
}
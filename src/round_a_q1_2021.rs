/*
A B C A A
0 1 2 3 4
1 2 3 4 5 N - i + 1 where 1 <= i <= N/2
 */
/*
Sample input:
2
5 1
ABCAA
4 2
ABAA

Sample output:
Case #1: 0
Case #2: 1
 */

pub fn compute(str_len: u32, target_score: u32, input_str: &str) -> u32 {
    let mut cur_score: u32 = 0;
    for i  in 0..str_len/2 {
        // let j = i + 1;
        let i = i + 1;
        let ii = str_len - i + 1;

        if input_str.as_bytes()[(i - 1) as usize] != input_str.as_bytes()[(ii - 1) as usize] {
            cur_score += 1;
        }
    }

    if cur_score == target_score {
        return 0;
    } else if cur_score < target_score {
        return target_score - cur_score;
    } else {
        return cur_score - target_score;
    }
}

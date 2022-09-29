use std::io::{self, BufRead};

pub fn google_kickstart_io() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let total_num_t = iterator.next().unwrap().unwrap();
    let total_num_t: u32 = total_num_t.parse().unwrap();
    for num_t in 1..=total_num_t {
        let input_one = iterator.next().unwrap().unwrap();
        let parts: Vec<&str> = input_one.split(" ").collect();
        let str_len: u32 = parts[0].parse().unwrap();
        let target_score: u32 = parts[1].parse().unwrap();
        let input_two = iterator.next().unwrap().unwrap();
        let input_str: String = input_two.parse().unwrap();

        // Below is to be used in different questions:
        // let res = compute(str_len, target_score, &input_str[..]);
        // println!("Case #{}: {}", num_t, res);
    }
}
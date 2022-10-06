struct Solution;
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();
        let mut track: Vec<i32> = Vec::new();

        Solution::do_solution(0, &graph, &mut results, &mut track);

        results
    }

    fn do_solution(curr_node: i32, graph: &Vec<Vec<i32>>, results: &mut Vec<Vec<i32>>, track: &mut Vec<i32>) -> () {
        track.push(curr_node);

        if curr_node as usize == graph.len() - 1 {
            results.push(track.clone());
        }

        for n in &graph[curr_node as usize] {
            Solution::do_solution(*n, graph, results, track)
        }

        track.pop();
    }
}

#[cfg(test)]
mod all_paths {

    #[test]
    fn test_one() {
        use super::*;
        let r0 = vec![4,3,1];
        let r1 = vec![3,2,4];
        let r2 = vec![3];
        let r3 = vec![4];
        let r4 = vec![];
        let graph: Vec<Vec<i32>> = vec![r0, r1, r2, r3, r4];
        let s = Solution::all_paths_source_target(graph);
        dbg!(s);
    }
}
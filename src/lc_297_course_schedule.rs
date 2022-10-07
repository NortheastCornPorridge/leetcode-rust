use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let graph = Self::build_graph(&prerequisites);

        let mut visited: Vec<bool> = vec![false; num_courses as usize];
        let mut on_path: Vec<bool> = vec![false; num_courses as usize];


        Self::do_can_finish(0, &graph, &mut visited, &mut on_path)
    }

    fn do_can_finish(current: i32,
                     graph: &HashMap<i32, Vec<i32>>, 
                     visited: &mut Vec<bool>, 
                     onPath: &mut Vec<bool>) -> bool 
    {

        if visited[current] {} 

        if let Some(neighbours) = graph.get(&current) {

        } else {}

        false
    }

    fn build_graph(prerequisites: &Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        for pair in prerequisites {
            // I don't know how to fix this linting error?
            // If pair[0] exits, it won't insert a new value
            // with vec![pair[1]]
            // graph.entry(pair[0]).or_insert(vec![pair[1]]);
            // let neighbours = graph.get_mut(&pair[0]).expect("key does not exist");
            // neighbours.push(pair[0]);

            // [0, 1]
            // [math2, math1]
            // 1 -> [0]
            // math1 -> [math2]
            if let Some(neighbours) = graph.get_mut(&pair[1]) {
                neighbours.push(pair[0]);
            } else {
                graph.insert(pair[1], vec![pair[0]]);
            }
        }

        graph
    }
}

#[cfg(test)]
mod course_schedule_one_test {
    #[test]
    fn test_graph() {
        use super::*;
        let prereq = vec![
            vec![1, 0],
            vec![2, 0],
            vec![3, 1],
            vec![4, 3]
        ];
        let graph = Solution::build_graph(&prereq);
        dbg!(graph);
    }
}
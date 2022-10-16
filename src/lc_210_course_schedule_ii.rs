use std::collections::{HashMap, VecDeque};
struct Solution;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let graph = Self::build_graph(&prerequisites);
        let mut visited: Vec<bool> = vec![false; num_courses as usize];
        let mut on_path: Vec<bool> = vec![false; num_courses as usize];
        let mut has_cycle: bool = false;
        let mut track: Vec<i32> = vec![];
        for c in graph.keys() {
            Self::do_find_order(num_courses,
                                *c,
                                &mut track,
                                &mut has_cycle,
                                &graph,
                                &mut visited,
                                &mut on_path);
        }
        track.reverse();
        track
    }

    fn do_find_order(num_courses: i32,
                     current: i32,
                     track: &mut Vec<i32>,
                     has_cycle: &mut bool,
                     graph: &HashMap<i32, Vec<i32>>,
                     visited: &mut Vec<bool>,
                     on_path: &mut Vec<bool>)
    {
        if on_path[current as usize] {
            *has_cycle = true;
            return
        }

        if visited[current as usize]  || *has_cycle {
            return
        }

        visited[current as usize] = true;
        on_path[current as usize] = true;
        if let Some(neighbours) = graph.get(&current) {
            for n in neighbours {
                Self::do_find_order(num_courses, *n, track, has_cycle, graph, visited, on_path);
            }
        }
        track.push(current);
        on_path[current as usize] = false;
    }

    fn build_graph(prerequisites: &Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for pair in prerequisites {
            if let Some(neighbours) = graph.get_mut(&pair[1]) {
                neighbours.push(pair[0]);
            } else {
                graph.insert(pair[1], vec![pair[0]]);
            }
        }
        graph
    }
}

use std::collections::HashSet;
use std::collections::HashMap;


pub fn compute_max_weight_matching(left_list: Vec<usize>, right_list: Vec<usize>, preferences: &[Vec<f64>], match_size: usize) -> (f64, HashSet<usize>) {
    let mut weights: Vec<Vec<i128>> = vec![vec![0; right_list.len()]; left_list.len()];
    let mut right_map: HashMap<usize, usize> = HashMap::new();

    // println!("left_list,right_list {:?},{:?}",left_list,right_list);

    assert!(left_list.len() <= right_list.len(), "Number of rows must be less than or equal to number of columns.");

    for (i, &left) in left_list.iter().enumerate() {
        for (j, &right) in right_list.iter().enumerate() {
            right_map.insert(j, right_list[j]);
            weights[i][j] = (preferences[left][right]*(10000000.0)) as i128;
        }
    }

    let (max_weight, matched_list) = fixed_size_max_weight_matching(&weights, match_size);
    let mut assignments: HashSet<usize> = HashSet::new();
    for i in matched_list {
        assignments.insert(right_map[&i]);
    }
    ((max_weight as f64)/(10000000.0), assignments)
}

fn bellman_ford(graph: &Vec<Vec<(usize, i128)>>, start: usize, end: usize) -> Option<(i128, Vec<usize>)> {
    let graph_len = graph.len();
    let mut distance = vec![i128::MAX; graph_len];
    let mut predecessor = vec![None; graph_len];
    distance[start] = 0;

    for _ in 0..graph_len - 1 {
        let mut updated = false;
        for u in 0..graph_len {
            if distance[u] == i128::MAX {
                continue;
            }
            for &(v, weight) in &graph[u] {
                if distance[u] + weight < distance[v] {
                    distance[v] = distance[u] + weight;
                    predecessor[v] = Some(u);
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }

    if distance[end] == i128::MAX {
        return None;
    }

    let mut path = vec![];
    let mut current = end;
    while let Some(prev) = predecessor[current] {
        path.push(current);
        current = prev;
        if current == start {
            break;
        }
    }
    path.push(start);
    path.reverse();

    Some((distance[end], path))
}

// Function to find the maximum weight matching of a fixed size
fn fixed_size_max_weight_matching(weights: &[Vec<i128>], k: usize) -> (i128, Vec<usize>) {
    let n = weights.len();
    let m = weights[0].len();

    assert!(n <= m, "Number of rows must be less than or equal to number of columns.");
    assert!(k <= n, "k must be less than or equal to number of rows.");

    let mut matched_left = vec![usize::MAX; n];
    let mut matched_right = vec![usize::MAX; m];
    let mut max_weight = 0;

    // Updating the matching increasing the size from 1 to k.
    for size in 1..=k {

        let mut g = vec![vec![]; n + m + 2];
        for i in 0..n {
            for j in 0..m {
                if matched_left[i] == usize::MAX && matched_right[j] == usize::MAX {
                    g[n + m].push((i, 0));
                    g[i].push((j + n, -1 * weights[i][j]));
                    g[j + n].push((n + m + 1, 0));
                } else if matched_left[i] == usize::MAX && matched_right[j] != usize::MAX {
                    g[n + m].push((i, 0));
                    g[i].push((j + n, -1 * weights[i][j]));
                } else if matched_left[i] != usize::MAX && matched_right[j] == usize::MAX {
                    g[i].push((j + n, -1 * weights[i][j]));
                    g[j + n].push((n + m + 1, 0));
                } else {
                    if matched_left[i] == j && matched_right[j] == i {
                        g[j + n].push((i, weights[i][j])); // Edges included in the matching are reversed and their weights are set to positive.
                    }
                }
            }
        }
        // println!("matched_left {:?}", matched_left);
        // println!("matched_right {:?}", matched_right);
        // println!("g {:?}", g);
        //let (max_end, max_path_weight) = max_alternating_path(&g, &matched_left, &matched_right);
        

        let start = n + m;
        let end = n + m + 1;
        if let Some((cost, path)) = bellman_ford(&g, start, end) {
            // println!("The cost of the shortest path is: {}", -cost);
            // println!("Path: {:?}", path);

            for i in 1..(path.len() - 1) {
                let x = path[i];
                let y = path[i + 1];
                if x != n + m && y != m + n + 1 {
                    if x < n && n - 1 < y {
                        matched_left[x] = y-n;
                        matched_right[y-n] = x;
                        max_weight += weights[x][y-n];
                    } else if y < n && n - 1 < x {
                        max_weight -= weights[y][x-n];
                    }
                }
            }
            
        } else {
            println!("No augmenting path found or negative cycle detected");
            break;
        }
        // println!("matched_left {:?}", matched_left);
        // println!("matched_right {:?}", matched_right);

        // println!("maximum weight of matching with {} edges {}", size, max_weight);
        let mut l_num = 0;
        for i in 0..matched_left.len() {
            if matched_left[i] != usize::MAX {
                l_num += 1;
            }
        }
        let mut r_num = 0;
        for j in 0..matched_right.len() {
            if matched_right[j] != usize::MAX {
                r_num += 1;
            }
        }
        assert!(size == l_num && size == r_num, "l_num and r_num must be the same as size k.")
    }
    
    let right_matched: Vec<usize> = matched_right.iter().enumerate().filter_map(|(i, &v)| if v != usize::MAX { Some(i) } else { None }).collect();
    // println!("max_weight {:?}", max_weight);
    // println!("right_matched {:?}", right_matched);
    (max_weight, right_matched)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kuhn_munkres() {

        use pathfinding::kuhn_munkres::*;
        use pathfinding::{matrix,matrix::{Matrix}};

        let mut m = Matrix::new(2, 2, 0);
        println!("{:?}",m);
        m[(0, 1)] = (1.0*(100000.0)) as i128;
        m[(1, 0)] = (10.0*(100000.0)) as i128;
        m[(1, 1)] = (11.0*(100000.0)) as i128;
        println!("{:?}",m);


        //println!("Maximum weight matching: {:?}", kuhn_munkres(&matrix![[1]]));
        println!("Maximum weight matching: {:?}", kuhn_munkres(&m));

        let max_matching = kuhn_munkres(&m);
        let mut max_val = 0;
        for i in 0..2 as usize{
            println!("{:?}",max_matching.1[i]);
            max_val += m[(i, max_matching.1[i])];
        }
        assert_eq!(max_matching.0, max_val);
    }

    #[test]
    fn test_fixed_size_max_weight_matching() {
        let weights = vec![
        vec![10, 2, 3],
        vec![4, 15, 6]
        ];
        println!("{:?}",weights);
        let (max_weight, matched) = fixed_size_max_weight_matching(&weights, 2);
        println!("Max weight: {}", max_weight);
        println!("Matched indices: {:?}", matched);
        assert_eq!(max_weight, 25);


        let weights = vec![
            vec![10, 2, 3],
            vec![4, 10, 21],
            vec![7, 21, 30]
        ];
        let (max_weight, matched) = fixed_size_max_weight_matching(&weights, 3);
        println!("Max weight: {}", max_weight);
        println!("Matched indices: {:?}", matched);
        assert_eq!(max_weight, 52);
    }


    #[test]
    fn test_compute_max_weight_matching() {
        use rand::Rng;
        
        let n_each: usize = 10;
        let num_groups: usize = 10;
        let num_items: usize = 100;


        let mut groups: Vec<Vec<usize>> = Vec::new();
        for i in 0..num_groups {
            groups.push((0..n_each).map(|j| i * n_each + j).collect());
        }

        let num_agents = n_each * num_groups;

        let mut preferences: Vec<Vec<f64>> = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..num_agents {
            preferences.push((0..num_items).map(|_| rng.gen()).collect());
        }

        let match_size = 1;

        let (max_weight, assignments) = compute_max_weight_matching((0..n_each).collect(), (50..num_items).collect(), &preferences, match_size);

        println!("max_weight: {:?}", max_weight);
        println!("assignments: {:?}", assignments);
        assert_eq!(assignments.len(),match_size);
    }
}
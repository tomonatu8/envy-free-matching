use crate::matching::compute_max_weight_matching;
use std::collections::HashSet;
use std::collections::HashMap;


pub fn round_robin_allocation_by_group(num_items: usize, num_groups: usize, n_each: usize, groups: &Vec<Vec<usize>>, preferences: &Vec<Vec<f64>>) -> (Vec<HashSet<usize>>, Vec<f64>) {
    // let mut allocation: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut allocation: Vec<HashSet<usize>> = vec![HashSet::new(); num_groups];
    // println!("allocation {:?}",allocation);

    let mut utility_list: Vec<f64> = vec![0.0; num_groups];

    let mut available_items: Vec<usize> = (0..num_items).collect();

    let mut selected: HashMap<usize, bool> = HashMap::new();
    for p in 0..num_groups {
        selected.insert(p, false);
    }

    let mut match_size = 0;

    while !available_items.is_empty() {

        match_size += 1;

        // println!("---allocation {:?}", allocation);
        // println!("---available_items {:?}", available_items);
        // println!("---round {:?}", match_size);

        let mut terminate = true;
        for p in 0..num_groups {
            if allocation[p].len() < n_each {
                terminate = false;
            }
        }
        if terminate {
            break;
        }


        for p in 0..num_groups {

            // println!("---Class {:?}", p);

            if allocation[p].len() == n_each {
                continue;
            }

            let mut aval_items_for_each_group = available_items.clone();
            if !&allocation[p].is_empty() {
                aval_items_for_each_group.extend(&allocation[p]);
            }
            // println!("aval_items_for_each_group {:?}", aval_items_for_each_group);

            let (max_weight, assignments) = compute_max_weight_matching(groups[p].clone(), aval_items_for_each_group.clone(), &preferences, match_size);

            // println!("max_weight {:?}", max_weight);

            let new_allocation_p = assignments;
            let diff: HashSet<usize> = allocation[p].symmetric_difference(&new_allocation_p).cloned().collect();
            
            // println!("allocation[p] {:?}", allocation[p]);
            // println!("new_allocation_p {:?}", new_allocation_p);

            // println!("diff {:?}",diff);
            // println!("diff.len() {}",diff.len());
            assert!(diff.len() == 1, "diff.len() == 1");

            allocation[p] = new_allocation_p;
            utility_list[p] = max_weight;

            if !diff.is_empty() {
                if let Some(&item) = diff.iter().next() {
                    if let Some(pos) = available_items.iter().position(|&x| x == item) {
                        available_items.remove(pos);
                    }
                }
            } 
        }
    }
    (allocation, utility_list)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_robin_allocation_by_group() {

        use rand::Rng;
        
        let n_each: usize = 20;
        let num_groups: usize = 5;
        let num_items: usize = 200;


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

        //println!("groups: {:?}", groups);

        let (allocation, utility_list) = round_robin_allocation_by_group(num_items, num_groups, n_each, &groups, &preferences);
        println!("allocation \n {:?}", allocation);
        println!("utility_list \n {:?}", utility_list);
        }
}

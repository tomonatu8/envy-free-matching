use std::collections::HashSet;
use std::collections::HashMap;
use rand::Rng;

fn round_robin_allocation_by_group(num_items: usize, groups: &Vec<Vec<usize>>, preferences: &Vec<Vec<f64>>) -> HashMap<usize, Vec<usize>> {
    let mut allocation: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut available_items: HashSet<usize> = (0..num_items).collect();

    for group in groups.iter() {
        if group.is_empty() {
            break;
        }

        let mut best_choice: Option<(usize, usize)> = None;
        let mut best_value = -1.0;

        for &agent in group.iter() {
            if let Some(&best_item_for_agent) = available_items.iter().max_by(|&&x, &&y| preferences[agent][x].partial_cmp(&preferences[agent][y]).unwrap()) {
                if preferences[agent][best_item_for_agent] > best_value {
                    best_choice = Some((agent, best_item_for_agent));
                    best_value = preferences[agent][best_item_for_agent];
                }
            }
        }

        if let Some((agent, item)) = best_choice {
            allocation.entry(agent).or_insert(Vec::new()).push(item);
            available_items.remove(&item);
        }
    }

    allocation
}


fn main() {
    let n_each = 2;
    let k = 3;
    let num_items = 5;

    // グループの生成
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for i in 0..k {
        groups.push((0..n_each).map(|j| i * n_each + j).collect());
    }

    // エージェントの好みの生成
    let num_agents = n_each * k;
    let mut preferences: Vec<Vec<f64>> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..num_agents {
        preferences.push((0..num_items).map(|_| rng.gen()).collect());
    }

    // アイテムの割り当て（この関数は別途定義が必要）
    let allocation = round_robin_allocation_by_group(num_items, &groups, &preferences);

    // 割り当て結果の表示（デバッグ用）
    println!("{:?}", allocation);
}
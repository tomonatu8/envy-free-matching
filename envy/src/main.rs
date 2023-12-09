use std::collections::HashSet;
use std::collections::HashMap;
use rand::Rng;

use pathfinding::kuhn_munkres::*;
use pathfinding::{matrix,matrix::{Matrix}};

fn round_robin_allocation_by_group(num_items: usize, groups: &Vec<Vec<usize>>, preferences: &Vec<Vec<f64>>) -> HashMap<usize, Vec<usize>> {
    let mut allocation: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut available_items: HashSet<usize> = (0..num_items).collect();

    for group in groups.iter() {
        for &agent in group.iter() {
            allocation.entry(agent).or_insert(Vec::new());
        }
    }

    // 各エージェントがアイテムを選択したかどうかを追跡する
    let mut selected: HashMap<usize, bool> = HashMap::new();
    for group in groups.iter() {
        for &agent in group.iter() {
            selected.insert(agent, false);
        }
    }

    while !available_items.is_empty() {
        let mut all_agents_selected = true;

        for group in groups.iter() {
            if group.is_empty() {
                break;
            }

            let mut best_choice: Option<(usize, usize)> = None;
            let mut best_value = -1.0;

            for &agent in group.iter() {
                // このエージェントが既にアイテムを選択していたらスキップ
                if *selected.get(&agent).unwrap_or(&false) {
                    continue;
                }

                all_agents_selected = false;

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
                selected.insert(agent, true); // エージェントがアイテムを選択したことを記録
            }
        }
        if all_agents_selected {
            break;
        }
    }

    allocation
}


fn main() {
    let n_each = 100;
    let k = 2;
    let num_items = 200;

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

    println!("{:?}", groups);
    // println!("{:?}", preferences);

    // アイテムの割り当て（この関数は別途定義が必要）
    let allocation = round_robin_allocation_by_group(num_items, &groups, &preferences);

    // 割り当て結果の表示（デバッグ用）
    println!("{:?}", allocation);


    let mut groups_util: Vec<Vec<usize>> = Vec::new();

    for i in 0..k {
        groups_util.push((0..n_each).map(|j| i * n_each + j).collect());
    }
    let mut utility_list: Vec<f64> = Vec::new();
    for group in groups_util.iter() {
        let mut utility: f64 = 0.0;
        for &agent in group.iter() {
            if let Some(&item) = allocation.get(&agent).and_then(|a| a.first()) {
                utility += preferences[agent][item];
            }
        }
        utility_list.push(utility);
    }
    println!("{:?}", utility_list);



    // 二部グラフのノードとエッジを定義
    // let m = &matrix![
    //     [7, 53, 183, 439, 863],
    //     [497, 383, 563, 79, 973],
    //     [287, 63, 343, 169, 583],
    //     [627, 343, 773, 959, 943],
    //     [767, 473, 103, 699, 303],
    // ];
    let mut m = Matrix::new(5, 5, 0);
    println!("{:?}",m[(0,0)]);
    m[(0, 0)] = 10;
    println!("{:?}",m[(0,0)]);
    m[(0, 1)] = 1;
    m[(1, 0)] = 10;
    m[(1, 1)] = 11;
    m[(0, 1)] = 2;
    // // 結果の表示
    println!("Maximum weight matching: {:?}", kuhn_munkres(&matrix![[1]]));
    println!("Maximum weight matching: {:?}", kuhn_munkres(&m));
    //println!("{:?}",m[(0,4)]+m[(1,1)]+m[(2,2)]+m[(3,3)]+m[(4,0)]);
}
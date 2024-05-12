use std::collections::HashSet;
use std::collections::HashMap;
use rand::Rng;
use csv;
use std::fs;
use std::fs::File;
use csv::Writer;
use std::env;
use indicatif::{ProgressBar};
use std::process;

mod config;
mod allocation;
mod matching;

use crate::config::Config;
use crate::allocation::round_robin_allocation_by_group;
use crate::matching::compute_max_weight_matching;



fn create_wtr(n_each: usize, num_groups: usize, num_items: usize) -> (Writer<File>, Writer<File>){
    let file_name_p = format!(
        "outcome/outcome_p_{}_{}_{}.csv",
        n_each,
        num_groups,
        num_items,
    );
    let file_out_p = fs::File::options()
        .write(true)
        .create(true)
        .open(file_name_p)
        .expect("CSV write failure");
    let wtr_p = csv::Writer::from_writer(file_out_p);

    let file_name_pq = format!(
        "outcome/outcome_pq_{}_{}_{}.csv",
        n_each,
        num_groups,
        num_items,
    );
    let file_out_pq = fs::File::options()
        .write(true)
        .create(true)
        .open(file_name_pq)
        .expect("CSV write failure");
    let wtr_pq = csv::Writer::from_writer(file_out_pq);

    (wtr_p, wtr_pq)
}

fn create_groups_pref(n_each: usize, num_groups: usize, num_agents: usize, num_items: usize) -> (Vec<Vec<usize>>, Vec<Vec<f64>>) {

    let mut groups: Vec<Vec<usize>> = Vec::new();
    for i in 0..num_groups {
        groups.push((0..n_each).map(|j| i * n_each + j).collect());
    }

    let mut preferences: Vec<Vec<f64>> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..num_agents {
        preferences.push((0..num_items).map(|_| rng.gen()).collect());
    }

    (groups, preferences)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //let n_each: usize = 10;
    //let num_groups: usize = 4;
    //let num_items: usize = 100;
    let num_agents = config.n_each * config.num_groups;
    let num_tries: usize = 100;

    let (mut wtr_p, mut wtr_pq) = create_wtr(config.n_each, config.num_groups, config.num_items);

    let pb = ProgressBar::new(num_tries as u64);
    
    for _ in 0..num_tries {
        //thread::sleep(Duration::from_millis(5));
        pb.inc(1);

        let (groups, preferences) = create_groups_pref(
            config.n_each, 
            config.num_groups,
            num_agents, 
            config.num_items,
        );
        //println!("groups: {:?}", groups);
        
        let (allocation, utility_list) = round_robin_allocation_by_group(
            config.num_items, 
            config.num_groups,
            config.n_each, 
            &groups, 
            &preferences,
        );
        // println!("allocation \n {:?}", allocation);
        // println!("utility_list \n {:?}", utility_list);
        
        for p in 0..config.num_groups {
            // println!("----------Class {} evaluates class {} 's bundle as {}.", p, p, utility_list[p]);
            //// println!("----------Class {} evaluates whole set of item as {}.", p, compute_max_weight_matching(groups_util[p].clone(), (0..num_items).collect(), &preferences).0);
            wtr_p.serialize(utility_list[p]).expect("CSV write failure");

            for q in 0..config.num_groups {
                // println!("{:?}",bundle_q);
                let (max_weight, assign) = compute_max_weight_matching(
                    groups[p].clone(), 
                    allocation[q].clone().into_iter().collect(), 
                    &preferences, 
                    config.n_each,
                );

                println!("Class {} evaluates class {}'s bundle as {}.", p, q, max_weight);

                if q == (p + 1)%config.num_groups {
                    wtr_pq.serialize(max_weight).expect("CSV write failure");
                }
            }
        }
    }
}



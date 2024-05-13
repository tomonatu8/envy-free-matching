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



fn create_wtr(n_each: usize, num_groups: usize, num_items: usize) -> (Writer<File>, Writer<File>, Writer<File>, Writer<File>){
    let make_file = |str: String| -> Writer<File>{
        let file_name = format!(
            "outcome/outcome_{}_{}_{}_{}.csv",
            str,
            n_each,
            num_groups,
            num_items,
        );
        let file_out = fs::File::options()
            .write(true)
            .create(true)
            .open(file_name)
            .expect("CSV write failure");
        let wtr = csv::Writer::from_writer(file_out);
        wtr
    };
    let wtr_p = make_file("p".to_string());
    let wtr_pq = make_file("pq".to_string());
    let wtr_diff_p = make_file("diff_p".to_string());
    let wtr_diff_pq = make_file("diff_pq".to_string());
    (wtr_p, wtr_pq, wtr_diff_p, wtr_diff_pq)
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


fn calculate_diff (num_1:f64, num_2:f64) -> f64 {
    if num_1 >= num_2 {
        (num_1 - num_2).powi(2)
    }
    else {
        0.0
    }
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

    let (mut wtr_p, mut wtr_pq, mut wtr_diff_p, mut wtr_diff_pq) = create_wtr(config.n_each, config.num_groups, config.num_items);

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


        let mut difference_p = 0.0;
        let mut difference_pq = 0.0;
        let mut count = 0;
        for i in 0..num_agents {
            for j in 0..config.num_items {

                let mut another_preferences: Vec<Vec<f64>> = vec![Vec::new(); num_agents];

                for k in 0..num_agents {
                    another_preferences[k] = preferences[k].clone();
                }
                // println!("----{}, {}----",i,j);
                another_preferences[i][j] = 0.0;

                let (another_allocation, another_utility_list) = round_robin_allocation_by_group(
                    config.num_items, 
                    config.num_groups,
                    config.n_each, 
                    &groups, 
                    &another_preferences,
                );

                // println!("another_allocation {:?}",another_allocation);

                let mut count_bool = false;
                //for p in 0..config.num_groups {
                    //if p!=0 {continue;}
                let each_diff_p = calculate_diff(utility_list[0], another_utility_list[0]);
                difference_p += each_diff_p;
                // println!("each_diff of {} : {}", p, each_diff_p);
                if each_diff_p != 0.0 { count_bool = true };
    
                    //for q in 0..config.num_groups {
                        //if q!=1 {continue;}
                let (max_weight, assign) = compute_max_weight_matching(
                    groups[0].clone(), 
                    allocation[1].clone().into_iter().collect(), 
                    &preferences, 
                    config.n_each,
                );

                let (another_max_weight, another_assign) = compute_max_weight_matching(
                    groups[0].clone(), 
                    another_allocation[1].clone().into_iter().collect(), 
                    &another_preferences, 
                    config.n_each,
                );
                let each_diff_pq =  calculate_diff(max_weight, another_max_weight);
                difference_pq += each_diff_pq;
                        //println!("each_diff of {} toward {} : {}", p, q, each_diff_pq);
                        // if q == (p + 1)%config.num_groups {
                        //     let each_diff_pq = max_weight - another_max_weight;
                        //     difference_pq += each_diff_pq;
                        //     println!("each_diff of {} toward {} : {}", p, q, each_diff_pq);
                        // }
                    
                if count_bool {
                    count += 1;
                }
            }
        }
        println!("count : {}", count);
        // if count > num_agents{
        //     break;
        // }

        println!("difference_p, difference_pq : {}, {}", difference_p, difference_pq);
        wtr_diff_p.serialize(difference_p).expect("CSV write failure");
        wtr_diff_pq.serialize(difference_pq).expect("CSV write failure");
        
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



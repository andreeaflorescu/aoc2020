use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

fn count_outer_bags_for(
    input: HashMap<String, HashMap<String, usize>>,
    input_color: String,
    containers: &mut HashSet<String>,
) {
    let mut additional_color_containers = Vec::new();

    for (color, bag_container) in &input {
        if bag_container.contains_key(&input_color) {
            containers.insert(color.clone());
            additional_color_containers.push(color.to_string())
        }
    }

    for additional_color in additional_color_containers.clone() {
        count_outer_bags_for(input.clone(), additional_color, containers);
    }
}

fn count_inner_bags_for(
    input: HashMap<String, HashMap<String, usize>>,
    input_color: String,
) -> usize {
    let mut all_count = 1;
    if let Some(bag_container) = input.clone().get(&input_color) {
        for (bag, count) in bag_container {
            all_count = all_count + count * count_inner_bags_for(input.clone(), bag.to_string());
        }
        return all_count;
    }

    return all_count;
}

fn get_matches(input: String) -> HashMap<String, HashMap<String, usize>> {
    let mut output = HashMap::new();
    let regex_all = Regex::new(r"([\w ]+)bags contain(( [0-9]+ [\w ]+ bags?,?)+)\.").unwrap();
    let regex = Regex::new(r"([0-9]+) ([\w ]+) bags?,?").unwrap();
    for line in input.lines() {
        for full_capture in regex_all.captures_iter(line) {
            let container_bag = full_capture[1].trim().to_string();
            let what_it_contains = &full_capture[2].to_string();

            let mut containing_bags = HashMap::<String, usize>::new();
            for capture in regex.captures_iter(what_it_contains) {
                let count = capture[1].to_string();
                let color = capture[2].to_string();
                containing_bags.insert(color, count.parse::<usize>().unwrap());
            }
            output.insert(container_bag, containing_bags);
        }
    }

    output
}

pub fn solve_puzzle(path: &Path) {
    let input = read_to_string(path).unwrap();
    let res = get_matches(input);
    let mut count = HashSet::new();
    count_outer_bags_for(res.clone(), "shiny gold".to_string(), &mut count);
    println!("container bags={}", count.len());
    println!(
        "how many bags does the shinny bag need: {}",
        count_inner_bags_for(res, String::from("shiny gold")) - 1
    );
}

use std::str::FromStr;
use std::io::Error;
use std::iter::FromIterator;
use std::collections::{HashMap, HashSet};

use crate::utility::{parse_file_lines, partition};


pub fn solve() -> Result<(), Error> {
    let bag_specs: Vec<BagSpec> = parse_file_lines("data/day7.txt")?;
    let contained: BagContained = bag_specs.iter().cloned().collect();
    println!("{}", contained.count_leaves(String::from("shiny gold")));
    let containing: BagContaining = bag_specs.iter().cloned().collect();
    println!("{}", containing.total_contained(&String::from("shiny gold")) - 1);
    Ok(())
}


#[derive(Debug)]
#[derive(Clone)]
struct BagContained (HashMap<String, Vec<String>>);

impl FromIterator<BagSpec> for BagContained {
    fn from_iter<I: IntoIterator<Item=BagSpec>>(iter: I) -> Self {
        let mut map = HashMap::<String, Vec<String>>::new();
        for bag_spec in iter {
            for (_, child) in bag_spec.constituents {
                map.entry(child).or_insert_with(|| vec![]).push(bag_spec.color.clone())
            }
        }
        BagContained (map)
    }
}

impl BagContained {
    fn collect_leaves(&self, root: &String, seen: &mut HashSet<String>) -> () {
        if !seen.contains(root) {
            seen.insert(root.clone());
            match self.0.get(root) {
                Some(nodes) => for node in nodes {
                    self.collect_leaves(&node, seen)
                },
                None => (),
            }
        }
    }

    fn count_leaves(&self, root: String) -> usize {
        let mut seen = HashSet::new();
        self.collect_leaves(&root, &mut seen);
        seen.len() - 1
    }
}


#[derive(Debug)]
#[derive(Clone)]
struct BagContaining (HashMap<String, BagSpec>);

impl FromIterator<BagSpec> for BagContaining {
    fn from_iter<I: IntoIterator<Item=BagSpec>>(iter: I) -> Self {
        BagContaining (iter.into_iter().map(|bs| (bs.color.clone(), bs)).collect())
    }
}

impl BagContaining {
    fn total_contained(&self, root: &String) -> usize {
        match self.0.get(root) {
            Some(bag_spec) => 1usize + bag_spec.constituents.iter().map(
                |(count, color)| count * self.total_contained(color)
            ).sum::<usize>(),
            None => 1,
        }
    }
}


#[derive(Debug)]
#[derive(Clone)]
struct BagSpec {
    color: String,
    constituents: Vec<(usize, String)>,
}

fn parse_constituents(s: String) -> Vec<(usize, String)> {
    let mut constituents = vec![];
    for part in s.split(",") {
        // " 2 wavy olive bags." => (" 2 wavy olive", "s.")
        let (counted_color, _) = partition(&part, " bag");
        // " 2 wavy olive" => ("2", "wavy olive")
        let (count, color) = partition(&counted_color.trim(), " ");
        match (count.parse::<usize>(), color) {
            (Ok(i), Some(c)) => constituents.push((i, c)),
            _ => (),
        }
    }
    constituents
}

impl FromStr for BagSpec {
    type Err = ();

    /// Read a literal such as
    /// `clear chartreuse bags contain 3 dotted black bags, 2 wavy olive bags.`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (color, tail) = partition(&s, " bags contain ");
        match tail {
            Some(x) => if x == "no other bags." {
                Ok(BagSpec{color, constituents: vec![]})
            } else {
                Ok(BagSpec{color, constituents: parse_constituents(x)})
            },
            None => Ok(BagSpec{color, constituents: vec![]}),
        }
    }
}
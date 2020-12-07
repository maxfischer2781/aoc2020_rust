use std::str::FromStr;
use std::io::Error;
use std::iter::FromIterator;
use std::collections::HashSet;

use crate::utility::parse_file_lines;


pub fn solve() -> Result<(), Error> {
    let questionnaires: Vec<Questionnaire> = parse_file_lines("data/day6.txt")?;
    let groups_any = merge_groups(&questionnaires, false);
    println!("Group sum any {}", groups_any.iter().map(|q| q.0.len()).sum::<usize>());
    let groups_all = merge_groups(&questionnaires, true);
    println!("Group sum all {}", groups_all.iter().map(|q| q.0.len()).sum::<usize>());
    Ok(())
}

fn merge_groups(questionnaires: &Vec<Questionnaire>, overlap: bool) -> Vec<Questionnaire> {
    let mut compiled = vec![];
    let mut current: Option<Questionnaire> = None;
    for questionnaire in questionnaires.iter() {
        match (questionnaire.is_empty(), &current) {
            (true, None) => (),
            (true, Some(qs)) => {
                if !qs.is_empty() {compiled.push(current.unwrap().clone())};
                current = None
            },
            (false, None) => current = Some(questionnaire.clone()),
            (false, Some(qs)) => {
                current = Some(if overlap {qs.overlap(&questionnaire)} else {qs.merge(&questionnaire)})
            }
        }
    }
    if let Some(x) = current {
        if !x.is_empty() {compiled.push(x)}
    }
    compiled
}

#[derive(Debug)]
#[derive(Clone)]
struct Questionnaire (HashSet<char>);


impl Questionnaire {
    fn merge(&self, other: &Questionnaire) -> Questionnaire {
        Questionnaire(HashSet::from_iter(self.0.iter().chain(other.0.iter()).copied()))
    }

    fn overlap(&self, other: &Questionnaire) -> Questionnaire {
        let intersection: HashSet<_> = self.0.intersection(&other.0).copied().collect();
        Questionnaire(intersection)
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl FromStr for Questionnaire {
    type Err = ();

    /// Read a literal such as `abcy`
    fn from_str(s: &str)  -> Result<Self, Self::Err> {
        Ok(Questionnaire (HashSet::from_iter(s.chars())))
    }
}
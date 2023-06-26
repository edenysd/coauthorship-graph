use std::collections::HashMap;

use crate::types::Publication;

pub fn calculate_exclusivity_per_pub(
    pub_list: &Vec<Publication>,
) -> HashMap<(&String, &String), Vec<(usize, f64)>> {
    let mut exclusivity_per_pub = HashMap::<(&String, &String), Vec<(usize, f64)>>::new();

    let mut pub_index = 0;
    for publication in pub_list {
        let coauthors = &publication.coauthors;
        let number_of_authors = coauthors.len() as f64;

        for author_a in coauthors {
            for author_b in coauthors {
                if author_a < author_b {
                    let mut curr_vec: Vec<(usize, f64)> =
                        match exclusivity_per_pub.get(&(author_a, author_b)) {
                            Some(v) => v.to_vec(),
                            None => Vec::<(usize, f64)>::new(),
                        };
                    curr_vec.push((pub_index, 1.0 / (number_of_authors - 1.0)));

                    exclusivity_per_pub.insert((author_a, author_b), curr_vec.clone());
                    exclusivity_per_pub.insert((author_b, author_a), curr_vec);
                }
            }
        }
        pub_index += 1;
    }
    exclusivity_per_pub
}

pub fn calculate_co_authorship_freq<'a>(
    exclusivity_per_pub: &'a HashMap<(&String, &String), Vec<(usize, f64)>>,
) -> HashMap<(&'a String, &'a String), f64> {
    let mut co_authorship_freq = HashMap::<(&String, &String), f64>::new();

    for ((author_a, author_b), v) in exclusivity_per_pub {
        let mut sum = 0.0;
        for (_, exclusivity_value) in v {
            sum += exclusivity_value;
        }
        co_authorship_freq.insert((*author_a, *author_b), sum);
    }
    co_authorship_freq
}

pub fn calculate_total_co_authorship_freq_per_author<'a>(
    co_authorship_freq: &'a HashMap<(&String, &String), f64>,
) -> HashMap<&'a String, f64> {
    let mut total_co_authorship_freq_per_author = HashMap::<&String, f64>::new();

    for ((author_a, _), v) in co_authorship_freq {
        let mut cur_val = total_co_authorship_freq_per_author
            .get(author_a)
            .get_or_insert(&0.0)
            .clone();

        cur_val += v;
        total_co_authorship_freq_per_author.insert(author_a, cur_val);
    }
    total_co_authorship_freq_per_author
}

pub fn calculate_normalized_weights<'a>(
    co_authorship_freq: &'a HashMap<(&String, &String), f64>,
    total_co_authorship_freq_per_author: &'a HashMap<&String, f64>,
) -> HashMap<(&'a String, &'a String), f64> {
    let mut normalized_weights = HashMap::<(&String, &String), f64>::new();

    for ((author_a, author_b), v) in co_authorship_freq {
        normalized_weights.insert(
            (*author_a, *author_b),
            v / total_co_authorship_freq_per_author.get(author_a).unwrap(),
        );
    }
    normalized_weights
}

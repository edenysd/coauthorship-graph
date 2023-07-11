use std::collections::BTreeMap;

use crate::types::SimplePublication;

pub fn calculate_exclusivity_per_pub(
    pub_list: &Vec<SimplePublication>,
) -> BTreeMap<&String, BTreeMap<&String, Vec<(usize, f64)>>> {
    let mut exclusivity_per_pub = BTreeMap::<&String, BTreeMap<&String, Vec<(usize, f64)>>>::new();

    let mut pub_index = 0;
    for publication in pub_list {
        let coauthors = &publication.coauthors;
        let number_of_authors = coauthors.len() as f64;

        for author_a in coauthors {
            for author_b in coauthors {
                if author_a != author_b {
                    let curr_vec = match exclusivity_per_pub.get_mut(&author_a) {
                        Some(v) => match v.get_mut(&author_b) {
                            Some(v) => v,
                            None => {
                                v.insert(&author_b, Vec::<(usize, f64)>::new());
                                v.get_mut(&author_b).expect("failt to insert 2 key")
                            }
                        },
                        None => {
                            exclusivity_per_pub
                                .insert(&author_a, BTreeMap::<&String, Vec<(usize, f64)>>::new());
                            let temp_map = exclusivity_per_pub.get_mut(&author_a).unwrap();
                            temp_map.insert(&author_b, Vec::<(usize, f64)>::new());
                            temp_map.get_mut(&author_b).unwrap()
                        }
                    };
                    curr_vec.push((pub_index, 1.0 / (number_of_authors - 1.0)));
                }
            }
        }
        pub_index += 1;
        if pub_index % 100000 == 0 {
            println!("{}", pub_index);
        }
    }
    exclusivity_per_pub
}

pub fn calculate_co_authorship_freq<'a>(
    exclusivity_per_pub: &'a BTreeMap<&String, BTreeMap<&String, Vec<(usize, f64)>>>,
) -> BTreeMap<&'a String, BTreeMap<&'a String, f64>> {
    let mut co_authorship_freq = BTreeMap::<&String, BTreeMap<&String, f64>>::new();

    for (author_a, map) in exclusivity_per_pub {
        let mut sum = 0.0;

        for (author_b, v) in map {
            for (_, exclusivity_value) in v {
                sum += exclusivity_value;
            }

            if co_authorship_freq.contains_key(*author_a) == false {
                co_authorship_freq.insert(*author_a, BTreeMap::<&String, f64>::new());
            }
            co_authorship_freq
                .get_mut(*author_a)
                .unwrap()
                .insert(*&author_b, sum);
        }
    }
    co_authorship_freq
}

pub fn calculate_total_co_authorship_freq_per_author<'a>(
    co_authorship_freq: &'a BTreeMap<&'a String, BTreeMap<&'a String, f64>>,
) -> BTreeMap<&'a String, f64> {
    let mut total_co_authorship_freq_per_author = BTreeMap::<&String, f64>::new();

    for (author_a, map) in co_authorship_freq {
        for (_, v) in map {
            let mut cur_val = total_co_authorship_freq_per_author
                .get(author_a)
                .get_or_insert(&0.0)
                .clone();

            cur_val += v;
            total_co_authorship_freq_per_author.insert(author_a, cur_val);
        }
    }
    total_co_authorship_freq_per_author
}

pub fn calculate_normalized_weights<'a>(
    co_authorship_freq: &'a BTreeMap<&'a String, BTreeMap<&'a String, f64>>,
    total_co_authorship_freq_per_author: &'a BTreeMap<&String, f64>,
) -> BTreeMap<&'a String, BTreeMap<&'a String, f64>> {
    let mut normalized_weights = BTreeMap::<&String, BTreeMap<&String, f64>>::new();

    for (author_a, map) in co_authorship_freq {
        for (author_b, v) in map {
            if normalized_weights.contains_key(*author_a) == false {
                normalized_weights.insert(*author_a, BTreeMap::<&String, f64>::new());
            }
            normalized_weights.get_mut(*author_a).unwrap().insert(
                *author_b,
                v / total_co_authorship_freq_per_author.get(author_a).unwrap(),
            );
        }
    }
    normalized_weights
}

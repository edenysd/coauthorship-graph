use ustr::UstrMap;

use crate::types::SimplePublication;

//@TODO use Entry API instead custom checkers
pub fn calculate_exclusivity_per_pub_plus_calculate_co_authorship_freq(
    pub_list: Vec<SimplePublication>,
) -> UstrMap<UstrMap<f32>> {
    let mut exclusivity_per_pub = UstrMap::<UstrMap<f32>>::default();

    let mut pub_index: usize = 0;
    let mut amount_of_entries: usize = 0;
    let mut amount_of_pairs: usize = 0;
    for publication in pub_list {
        let coauthors = &publication.coauthors;
        let number_of_authors = coauthors.len() as f32;

        for author_a in coauthors {
            for author_b in coauthors {
                if author_a != author_b {
                    let cur_value = match exclusivity_per_pub.get_mut(&author_a) {
                        Some(v) => match v.get_mut(&author_b) {
                            Some(cur_value) => *cur_value,
                            None => {
                                v.insert(*author_b, 0.0);
                                0.0
                            }
                        },
                        None => {
                            amount_of_pairs += 1;
                            exclusivity_per_pub.insert(*author_a, UstrMap::<f32>::default());
                            let temp_map = exclusivity_per_pub.get_mut(&author_a).unwrap();
                            temp_map.insert(*author_b, 0.0);
                            0.0
                        }
                    };
                    exclusivity_per_pub
                        .get_mut(&author_a)
                        .unwrap()
                        .insert(*author_b, cur_value + 1.0 / (number_of_authors - 1.0));

                    amount_of_entries += 1;
                }
            }
        }
        pub_index += 1;
        if pub_index % 100000 == 0 {
            println!(
                "{} {} {} {}",
                pub_index,
                amount_of_entries,
                amount_of_pairs,
                ustr::num_entries()
            );
        }
    }
    exclusivity_per_pub
}

pub fn calculate_total_co_authorship_freq_per_author<'a>(
    co_authorship_freq: &UstrMap<UstrMap<f32>>,
) -> UstrMap<f32> {
    let mut total_co_authorship_freq_per_author = UstrMap::<f32>::default();
    let mut author_index = 0;
    for (author_a, map) in co_authorship_freq {
        for (_, v) in map {
            let mut cur_val = total_co_authorship_freq_per_author
                .get(&author_a)
                .get_or_insert(&0.0)
                .clone();

            cur_val += v;
            total_co_authorship_freq_per_author.insert(*author_a, cur_val);
        }
        author_index += 1;
        if author_index % 100000 == 0 {
            println!("{}", author_index);
        }
    }
    total_co_authorship_freq_per_author
}

pub fn calculate_normalized_weights<'a>(
    co_authorship_freq: UstrMap<UstrMap<f32>>,
    total_co_authorship_freq_per_author: UstrMap<f32>,
) -> UstrMap<UstrMap<f32>> {
    let mut normalized_weights = UstrMap::<UstrMap<f32>>::default();
    let mut author_index = 0;
    for (author_a, map) in co_authorship_freq {
        for (author_b, v) in map {
            if normalized_weights.contains_key(&author_a) == false {
                normalized_weights.insert(author_a, UstrMap::<f32>::default());
            }
            normalized_weights.get_mut(&author_a).unwrap().insert(
                author_b,
                v / total_co_authorship_freq_per_author.get(&author_a).unwrap(),
            );
        }
        author_index += 1;
        if author_index % 100000 == 0 {
            println!("{}", author_index);
        }
    }
    normalized_weights
}

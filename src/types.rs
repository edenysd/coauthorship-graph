use serde::Deserialize;
use ustr::Ustr;

#[derive(Debug, Deserialize, Clone)]
pub struct Publication {
    pub block_fullname: String,
    pub author_group_orcid: String,
    pub author_group_idx_in_block: String,
    pub citation_idx_in_author_group: String,
    pub doi: String,
    pub pid: String,
    pub author_position: u64,
    pub author_name: String,
    pub author_affiliation: String,
    #[serde(skip_deserializing)]
    pub coauthors: Vec<String>,
    pub coauthor_affliations: String,
    pub venue: String,
    pub pub_year: String,
    #[serde(skip_deserializing)]
    pub paper_title: String,
    #[serde(skip_deserializing)]
    pub paper_abstract: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SimplePublication {
    pub author_name: Ustr,
    #[serde(skip_deserializing)]
    pub coauthors: Vec<Ustr>,
}

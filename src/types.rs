use serde::Deserialize;

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
    pub paper_title: String,
    pub paper_abstract: String,
}

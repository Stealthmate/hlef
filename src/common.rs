#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PostingAmount {
    pub commodity: Option<String>,
    pub amount: String,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PostingLine {
    pub account: String,
    pub left_amount: Option<PostingAmount>,
    pub assertion: Option<String>,
    pub right_amount: Option<PostingAmount>,
    pub comment: Option<String>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LedgerLine {
    Empty,
    Comment(String),
    TransactionHead(String),
    Posting(PostingLine),
    PostingComment(String),
    Other(String),
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_amount_length")]
    pub amount_length: usize,
    #[serde(default = "default_account_max_length")]
    pub account_max_length: usize,
}

pub fn default_amount_length() -> usize {
    15
}
pub fn default_account_max_length() -> usize {
    70
}

impl Default for Config {
    fn default() -> Self {
        Config {
            amount_length: default_amount_length(),
            account_max_length: default_account_max_length(),
        }
    }
}

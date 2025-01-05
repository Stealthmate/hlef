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

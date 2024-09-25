#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PostingLine {
    pub account: String,
    pub commodity: Option<String>,
    pub equality: Option<String>,
    pub amount: Option<String>,
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

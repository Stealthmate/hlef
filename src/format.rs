use crate::common::{LedgerLine, PostingAmount, PostingLine};

const POSTING_AMOUNT_LENGTH: usize = 15;
const POSTING_ACCOUNT_MAX_LENGTH: usize = 70;

pub fn subtract(a: usize, b: usize) -> usize {
    let x = (a as isize) - (b as isize);
    if x < 1 {
        panic!("Not enough space: {a}, {b}")
    }
    x as usize
}

pub fn format_posting_account(account: &str) -> String {
    let mut out = String::new();
    out += account;
    out += &" ".repeat(subtract(POSTING_ACCOUNT_MAX_LENGTH, out.len()));

    out
}

pub fn format_posting_amount(pa: &PostingAmount) -> String {
    let mut out = "".to_string();
    out += pa.commodity.as_ref().map_or("", |x| x.as_str());

    let spacing_ = (POSTING_AMOUNT_LENGTH as i32) - (out.len() as i32) - (pa.amount.len() as i32);
    let spacing: usize = match spacing_.try_into() {
        Ok(x) if x > 2 => x,
        _ => panic!("Not enough space: {pa:#?}"),
    };

    out += &" ".repeat(spacing);
    out += &pa.amount;

    out
}

pub fn format_posting_line(posting: &PostingLine) -> String {
    let mut formatted = "  ".to_owned();
    formatted += &format_posting_account(&posting.account);
    formatted += "  ";
    formatted += &match &posting.left_amount {
        Some(pa) => format_posting_amount(pa),
        None => " ".repeat(POSTING_AMOUNT_LENGTH),
    };
    formatted += " ";
    formatted += &match &posting.assertion {
        Some(x) => format!("{: >5}", x),
        None => "     ".to_string(),
    };
    formatted += " ";
    formatted += &match &posting.right_amount {
        Some(pa) => format_posting_amount(pa),
        None => " ".repeat(POSTING_AMOUNT_LENGTH),
    };

    if let Some(comment) = &posting.comment {
        formatted += &format!(" ;{}", comment);
    }
    formatted.trim_end().to_owned()
}

pub fn format_line(line: &LedgerLine) -> String {
    match line {
        LedgerLine::Empty => "".to_owned(),
        LedgerLine::Comment(x) => format!(";{x}"),
        LedgerLine::TransactionHead(x) => x.clone(),
        LedgerLine::Posting(posting) => format_posting_line(posting),
        LedgerLine::PostingComment(x) => format!("  ;{x}"),
        LedgerLine::Other(x) => x.clone(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::{LedgerLine, PostingLine};

    #[test]
    fn test_it_formats_empty_line() {
        assert_eq!("".to_owned(), format_line(&LedgerLine::Empty))
    }

    #[test]
    fn test_it_formats_comment() {
        assert_eq!(
            ";foobar".to_owned(),
            format_line(&LedgerLine::Comment("foobar".to_owned()))
        )
    }

    #[test]
    fn test_it_formats_transaction_head() {
        assert_eq!(
            "2024-01-01 ! (T20240101-00) ; foobar".to_owned(),
            format_line(&LedgerLine::TransactionHead(
                "2024-01-01 ! (T20240101-00) ; foobar".to_owned()
            ))
        )
    }

    #[test]
    fn test_it_formats_posting() {
        let account = "asset:foobar".to_owned();
        assert_eq!(
            "  asset:foobar".to_owned(),
            format_line(&LedgerLine::Posting(PostingLine {
                account: account.clone(),
                left_amount: None,
                assertion: None,
                right_amount: None,
                comment: None
            }))
        );
        assert_eq!(
            "  asset:foobar                                                            JPY       10000".to_owned(),
            format_line(&LedgerLine::Posting(PostingLine {
                account: account.clone(),
                left_amount: Some(PostingAmount {
                    commodity: Some("JPY".to_string()),
                    amount: "10000".to_string()
                }),
                assertion: None,
                right_amount: None,
                comment: None
            }))
        );
        assert_eq!(
            "  asset:foobar                                                                                = JPY       10000".to_owned(),
            format_line(&LedgerLine::Posting(PostingLine {
                account: account.clone(),
                left_amount: None,
                assertion: Some("=".to_string()),
                right_amount: Some(PostingAmount {
                    commodity: Some("JPY".to_string()),
                    amount: "10000".to_string()
                }),
                comment: None
            }))
        );
        assert_eq!(
            "  asset:foobar                                                                                = JPY       10000 ; foo".to_owned(),
            format_line(&LedgerLine::Posting(PostingLine {
                account: account.clone(),
                left_amount: None,
                assertion: Some("=".to_string()),
                right_amount: Some(PostingAmount {
                    commodity: Some("JPY".to_string()),
                    amount: "10000".to_string()
                }),
                comment: Some(" foo".to_owned())
            }))
        );
        assert_eq!(
            "  asset:foobar                                                            JPY      123456   ==* JPY       10000 ; foo".to_owned(),
            format_line(&LedgerLine::Posting(PostingLine {
                account: account.clone(),
                left_amount: Some(PostingAmount {
                    commodity: Some("JPY".to_string()),
                    amount: "123456".to_string()
                }),
                assertion: Some("==*".to_string()),
                right_amount: Some(PostingAmount {
                    commodity: Some("JPY".to_string()),
                    amount: "10000".to_string()
                }),
                comment: Some(" foo".to_owned())
            }))
        );
    }

    #[test]
    fn test_it_formats_posting_comment() {
        assert_eq!(
            "  ;foobar".to_owned(),
            format_line(&LedgerLine::PostingComment("foobar".to_owned()))
        )
    }

    #[test]
    fn test_it_formats_other() {
        assert_eq!(
            "foobar".to_owned(),
            format_line(&LedgerLine::Other("foobar".to_owned()))
        )
    }
}

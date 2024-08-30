use crate::common::LedgerLine;

pub fn format_line(line: &LedgerLine) -> String {
    match line {
        LedgerLine::Empty => "".to_owned(),
        LedgerLine::Comment(x) => format!(";{x}"),
        LedgerLine::TransactionHead(x) => x.clone(),
        LedgerLine::Posting(posting) => {
            let mut formatted = "  ".to_owned();
            formatted += &format!("{: <70}", posting.account);

            let mut commodity = "".to_owned();
            if posting.equality {
                commodity += "= ";
            }
            commodity += &posting.commodity.clone().unwrap_or("".to_owned());
            formatted += &format!("{: >5}", commodity);

            formatted += &format!("{: >10}", &posting.amount.clone().unwrap_or("".to_owned()));
            if let Some(comment) = &posting.comment {
                formatted += &format!(" ;{}", comment);
            }
            formatted.trim_end().to_owned()
        }
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
                commodity: None,
                equality: false,
                amount: None,
                comment: None
            }))
        );
        assert_eq!(
            "  asset:foobar                                                            JPY"
                .to_owned(),
            format_line(&LedgerLine::Posting(PostingLine {
                account: account.clone(),
                commodity: Some("JPY".to_owned()),
                equality: false,
                amount: None,
                comment: None
            }))
        );
        assert_eq!(
            "  asset:foobar                                                            JPY     10000".to_owned(),
            format_line(&LedgerLine::Posting(PostingLine {
                account: account.clone(),
                commodity: Some("JPY".to_owned()),
                equality: false,
                amount: Some("10000".to_owned()),
                comment: None
            }))
        );
        assert_eq!(
            "  asset:foobar                                                          = JPY     10000".to_owned(),
            format_line(&LedgerLine::Posting(PostingLine {
                account: account.clone(),
                commodity: Some("JPY".to_owned()),
                equality: true,
                amount: Some("10000".to_owned()),
                comment: None
            }))
        );
        assert_eq!(
            "  asset:foobar                                                          = JPY     10000 ; foo".to_owned(),
            format_line(&LedgerLine::Posting(PostingLine {
                account: account.clone(),
                commodity: Some("JPY".to_owned()),
                equality: true,
                amount: Some("10000".to_owned()),
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

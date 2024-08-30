use crate::common::{LedgerLine, PostingLine};

const COMMENT_REGEX: &str = r"^;(.*)$";
const TRANSACTION_HEAD_REGEX: &str = r"^[0-9]{4}-[0-9]{2}-[0-9]{2}.*$";
const POSTING_REGEX: &str = r"^   *([^ ]+)(  +(\= +)?([^ ]+)? +([^ ;]+))? *(;(.*))?$";
const POSTING_COMMENT_REGEX: &str = r"^   *;(.*)$";

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    Fail(String, String),
}

fn parse_line_empty(line: &str) -> Result<LedgerLine, ParseError> {
    match line.trim() == "" {
        true => Ok(LedgerLine::Empty),
        false => Err(ParseError::Fail(line.to_owned(), "Not empty".to_owned())),
    }
}
fn parse_line_comment(line: &str) -> Result<LedgerLine, ParseError> {
    let re = regex::Regex::new(COMMENT_REGEX).unwrap();
    let Some(results) = re.captures(line) else {
        return Err(ParseError::Fail(
            line.to_owned(),
            "Could not parse comment".to_owned(),
        ));
    };

    Ok(LedgerLine::Comment(
        results.get(1).unwrap().as_str().trim_end().to_owned(),
    ))
}
fn parse_line_transaction_head(line: &str) -> Result<LedgerLine, ParseError> {
    let re = regex::Regex::new(TRANSACTION_HEAD_REGEX).unwrap();
    let Some(results) = re.captures(line) else {
        return Err(ParseError::Fail(
            line.to_owned(),
            "Could not parse transaction head".to_owned(),
        ));
    };

    Ok(LedgerLine::TransactionHead(
        results.get(0).unwrap().as_str().trim().to_owned(),
    ))
}
fn parse_line_posting(line: &str) -> Result<LedgerLine, ParseError> {
    let re = regex::Regex::new(POSTING_REGEX).unwrap();
    let Some(results) = re.captures(line) else {
        return Err(ParseError::Fail(
            line.to_owned(),
            "Could not parse posting".to_owned(),
        ));
    };

    Ok(LedgerLine::Posting(PostingLine {
        account: results.get(1).unwrap().as_str().to_owned(),
        equality: results.get(3).is_some(),
        commodity: results.get(4).map(|x| x.as_str().to_owned()),
        amount: results.get(5).map(|x| x.as_str().to_owned()),
        comment: results.get(7).map(|x| x.as_str().to_owned()),
    }))
}
fn parse_line_posting_comment(line: &str) -> Result<LedgerLine, ParseError> {
    let re = regex::Regex::new(POSTING_COMMENT_REGEX).unwrap();
    let Some(results) = re.captures(line) else {
        return Err(ParseError::Fail(
            line.to_owned(),
            "Could not parse posting comment".to_owned(),
        ));
    };

    Ok(LedgerLine::PostingComment(
        results.get(1).unwrap().as_str().trim_end().to_owned(),
    ))
}

pub fn parse_line(line: &str) -> Result<LedgerLine, ParseError> {
    for f in [
        parse_line_empty,
        parse_line_comment,
        parse_line_transaction_head,
        parse_line_posting,
        parse_line_posting_comment,
    ] {
        if let Ok(x) = f(line) {
            return Ok(x);
        }
    }

    Ok(LedgerLine::Other(line.to_owned()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_parses_empty_line() {
        for line in ["", "    "] {
            assert_eq!(Ok(LedgerLine::Empty), parse_line_empty(line))
        }
    }

    #[test]
    fn test_it_parses_comment() {
        for line in [";foo", ";foo     "] {
            assert_eq!(
                Ok(LedgerLine::Comment("foo".to_owned())),
                parse_line_comment(line)
            )
        }

        assert_eq!(
            Ok(LedgerLine::Comment("".to_owned())),
            parse_line_comment(";")
        )
    }

    #[test]
    fn test_it_parses_transaction_head() {
        for line in [
            "2024-01-01 ! (foo-bar) ; example",
            "2024-01-01 ! (foo-bar)",
            "2024-01-01 ! ; example",
            "2024-01-01 * (foo-bar) ; example",
        ] {
            assert_eq!(
                Ok(LedgerLine::TransactionHead(line.to_owned())),
                parse_line_transaction_head(line)
            )
        }
    }

    #[test]
    fn test_it_parses_posting() {
        for line in [
            "  asset:foobar  JPY 0",
            "     asset:foobar    JPY        0      ",
        ] {
            assert_eq!(
                Ok(LedgerLine::Posting(PostingLine {
                    account: "asset:foobar".to_owned(),
                    commodity: Some("JPY".to_owned()),
                    equality: false,
                    amount: Some("0".to_owned()),
                    comment: None
                })),
                parse_line_posting(line)
            )
        }

        for line in [
            "  asset:foobar  = JPY 0;example",
            "     asset:foobar  =  JPY        0      ;example",
        ] {
            assert_eq!(
                Ok(LedgerLine::Posting(PostingLine {
                    account: "asset:foobar".to_owned(),
                    commodity: Some("JPY".to_owned()),
                    equality: true,
                    amount: Some("0".to_owned()),
                    comment: Some("example".to_owned())
                })),
                parse_line_posting(line)
            )
        }
    }
    #[test]
    fn test_it_does_not_parse_transaction_head() {
        for line in ["  asset:foo  = JPY 0", "; example"] {
            assert!(parse_line_transaction_head(line).is_err())
        }
    }

    #[test]
    fn test_it_parses_posting_comment() {
        for line in ["  ;foo", "     ;foo", "  ;foo     "] {
            assert_eq!(
                Ok(LedgerLine::PostingComment("foo".to_owned())),
                parse_line_posting_comment(line)
            )
        }
    }
}

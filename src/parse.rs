use crate::common::{LedgerLine, PostingAmount, PostingLine};

const COMMENT_REGEX: &str = r"^;(.*)$";
const TRANSACTION_HEAD_REGEX: &str = r"^[0-9]{4}-[0-9]{2}-[0-9]{2}.*$";
const POSTING_ASSERTION_REGEX: &str = r"^([=*]+) *";
const WHITESPACE_REGEX: &str = r"^ *";
const COMMODITY_REGEX: &str = r"^[^=.,;*][^0-9 .,;]*";
const AMOUNT_REGEX: &str = r"^[\-0-9,.]+";
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

fn parse_posting_assertion(line: &mut String) -> Result<Option<String>, ParseError> {
    Ok(regex::Regex::new(POSTING_ASSERTION_REGEX)
        .unwrap()
        .captures(&line.clone())
        .map(|results| {
            *line = line[results.get(0).unwrap().len()..].to_string();
            results.get(1).unwrap().as_str().to_owned()
        }))
}

fn parse_posting_indent(line: &mut String) -> Result<(), ParseError> {
    let Some(ws) = regex::Regex::new(r"^   *")
        .unwrap()
        .captures(&line.clone())
        .map(|results| results.get(0).unwrap().as_str().len())
    else {
        return Err(ParseError::Fail(
            line.clone(),
            "Expected at least two spaces.".to_string(),
        ));
    };

    *line = line[ws..].to_string();
    Ok(())
}

fn parse_account(line: &mut String) -> Result<String, ParseError> {
    let Some(account) = regex::Regex::new(r"^[^ ]+")
        .unwrap()
        .captures(&line.clone())
        .map(|results| results.get(0).unwrap().as_str().to_owned())
    else {
        return Err(ParseError::Fail(
            line.clone(),
            "Expected account name.".to_string(),
        ));
    };

    *line = line[account.len()..].to_string();
    Ok(account)
}

fn parse_commodity(line: &mut String) -> Result<Option<String>, ParseError> {
    let commodity = regex::Regex::new(COMMODITY_REGEX)
        .unwrap()
        .captures(&line.to_owned())
        .map(|results| results.get(0).unwrap().as_str().to_owned());
    if let Some(x) = &commodity {
        *line = line[x.len()..].to_string();
    };

    Ok(commodity)
}

fn parse_amount(line: &mut String) -> Result<Option<String>, ParseError> {
    match regex::Regex::new(AMOUNT_REGEX)
        .unwrap()
        .captures(&line.to_owned())
        .map(|results| results.get(0).unwrap().as_str().to_owned())
    {
        Some(amount) => {
            *line = line[amount.len()..].to_string();
            Ok(Some(amount))
        }
        None => Ok(None),
    }
}

fn parse_whitespace(line: &mut String) -> Result<(), ParseError> {
    match regex::Regex::new(WHITESPACE_REGEX)
        .unwrap()
        .captures(&line.clone())
        .map(|results| results.get(0).unwrap().as_str().to_owned())
    {
        Some(ws) => {
            *line = line[ws.len()..].to_string();
            Ok(())
        }
        None => Err(ParseError::Fail(
            line.clone(),
            "Could not parse whitespace.".to_string(),
        )),
    }
}

fn parse_posting_amount(line: &mut String) -> Result<Option<PostingAmount>, ParseError> {
    let mut nested_line = line.clone();
    let commodity = parse_commodity(&mut nested_line)?;
    parse_whitespace(&mut nested_line)?;
    let amount = parse_amount(&mut nested_line)?;

    match (&commodity, &amount) {
        (None, None) => Ok(None),
        (Some(_), None) => Err(ParseError::Fail(
            line.clone(),
            "Found commodity but no amount.".to_string(),
        )),
        (_, Some(amount_)) => {
            *line = nested_line;
            Ok(Some(PostingAmount {
                commodity,
                amount: amount_.clone(),
            }))
        }
    }
}

fn parse_comment(line: &mut String) -> Result<Option<String>, ParseError> {
    Ok(regex::Regex::new(COMMENT_REGEX)
        .unwrap()
        .captures(&line.clone())
        .map(|results| {
            *line = line[results.get(0).unwrap().len()..].to_string();
            results.get(1).unwrap().as_str().to_owned()
        }))
}

fn parse_line_posting(line: &str) -> Result<LedgerLine, ParseError> {
    let mut nested_line = line.to_string().clone();

    parse_posting_indent(&mut nested_line)?;
    let account = parse_account(&mut nested_line)?;
    parse_whitespace(&mut nested_line)?;
    let left_amount = parse_posting_amount(&mut nested_line)?;
    parse_whitespace(&mut nested_line)?;
    let assertion = parse_posting_assertion(&mut nested_line)?;
    parse_whitespace(&mut nested_line)?;
    let right_amount = parse_posting_amount(&mut nested_line)?;
    parse_whitespace(&mut nested_line)?;
    let comment = parse_comment(&mut nested_line)?;

    Ok(LedgerLine::Posting(PostingLine {
        account,
        left_amount,
        assertion,
        right_amount,
        comment,
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
        parse_line_posting_comment,
        parse_line_posting,
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
    fn test_it_parses_commodity() {
        let jpy = Some("JPY".to_string());
        assert_eq!(Ok(jpy.clone()), parse_commodity(&mut "JPY".to_string()));
        assert_eq!(Ok(jpy.clone()), parse_commodity(&mut "JPY   ".to_string()));
    }

    #[test]
    fn test_it_parses_amount() {
        assert_eq!(
            Ok(Some("1,000,000".to_string())),
            parse_amount(&mut "1,000,000   ".to_string())
        );
        assert_eq!(
            Ok(Some("-1,000,000".to_string())),
            parse_amount(&mut "-1,000,000".to_string())
        );
        assert_eq!(
            Ok(Some("-1000000.0".to_string())),
            parse_amount(&mut "-1000000.0".to_string())
        );
    }

    #[test]
    fn test_it_parses_posting() {
        assert_eq!(
            Ok(LedgerLine::Posting(PostingLine {
                account: "expense:foo".to_owned(),
                left_amount: None,
                assertion: Some("=".to_owned()),
                right_amount: Some(PostingAmount {
                    commodity: Some("JPY".to_owned()),
                    amount: "0".to_owned()
                }),
                comment: None
            })),
            parse_line_posting("  expense:foo  = JPY 0")
        );

        for line in [
            "  asset:foobar  JPY 0",
            "     asset:foobar    JPY        0      ",
        ] {
            assert_eq!(
                Ok(LedgerLine::Posting(PostingLine {
                    account: "asset:foobar".to_owned(),
                    left_amount: Some(PostingAmount {
                        commodity: Some("JPY".to_owned()),
                        amount: "0".to_owned()
                    }),
                    assertion: None,
                    right_amount: None,
                    comment: None
                })),
                parse_line_posting(line)
            )
        }

        for line in ["  expense:foobar  JPY  123  ; 2025/04/01 12:34:56,foobar"] {
            assert_eq!(
                Ok(LedgerLine::Posting(PostingLine {
                    account: "expense:foobar".to_owned(),
                    left_amount: Some(PostingAmount {
                        commodity: Some("JPY".to_owned()),
                        amount: "123".to_owned()
                    }),
                    assertion: None,
                    right_amount: None,
                    comment: Some(" 2025/04/01 12:34:56,foobar".to_owned())
                })),
                parse_line_posting(line)
            )
        }

        for line in [
            "  asset:foobar  JPY -123 ==* JPY 0;example",
            "     asset:foobar  JPY -123 ==*  JPY        0      ;example",
        ] {
            assert_eq!(
                Ok(LedgerLine::Posting(PostingLine {
                    account: "asset:foobar".to_owned(),
                    left_amount: Some(PostingAmount {
                        commodity: Some("JPY".to_owned()),
                        amount: "-123".to_owned()
                    }),
                    assertion: Some("==*".to_owned()),
                    right_amount: Some(PostingAmount {
                        commodity: Some("JPY".to_owned()),
                        amount: "0".to_owned()
                    }),
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

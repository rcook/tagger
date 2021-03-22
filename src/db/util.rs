use crate::like::Like;

pub fn escape_string_literal(s: &str) -> String {
    wrap_and_escape(s, '\'')
}

pub fn make_like_expression(like: &Like) -> String {
    format!("LIKE {}", escape_string_literal(like.as_str()))
}

// See https://github.com/rusqlite/rusqlite/blob/master/src/pragma.rs#L138
fn wrap_and_escape(s: &str, quote: char) -> String {
    let mut buffer = String::new();
    buffer.push(quote);
    for c in s.chars() {
        if c == quote {
            buffer.push(c);
        }
        buffer.push(c)
    }
    buffer.push(quote);
    buffer
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use super::*;
    use crate::error::Result;

    #[test]
    fn test_escape_string_literal() {
        let sql = escape_string_literal("value'; --");
        assert_eq!("'value''; --'", sql);
    }

    #[test]
    fn test_make_like_expression() -> Result<()> {
        assert_eq!(
            "LIKE 'like'",
            make_like_expression(&Like::try_from("like")?)
        );
        assert_eq!(
            "LIKE '%like_'",
            make_like_expression(&Like::try_from("%like_")?)
        );
        assert_eq!(
            "LIKE '''like'''",
            make_like_expression(&Like::try_from("'like'")?)
        );
        Ok(())
    }
}

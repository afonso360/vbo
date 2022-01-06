#[derive(Debug, Clone, Hash, PartialEq)]
enum Token<'a> {
    SectionHeader(&'a str),
    Line(&'a str),
}

fn tokenize<'a>(text: &'a str) -> impl Iterator<Item = Token<'a>> {
    text.lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }

            Some(if line.starts_with('[') {
                Token::SectionHeader(line.trim_start_matches('[').trim_end_matches(']'))
            } else {
                Token::Line(line)
            })
        })
}
//
// #[derive(Debug, Clone)]
// enum Parser<'a> {
//
// }




#[cfg(test)]
mod tests {
    use std::io;
    use crate::parser::{Token, tokenize};

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("File created on 07/09/2017 @ 15:58:57").collect::<Vec<_>>(),
            vec![Token::Line("File created on 07/09/2017 @ 15:58:57")]
        );

        assert_eq!(
            tokenize("[section header]").collect::<Vec<_>>(),
            vec![Token::SectionHeader("section header")]
        );

        assert_eq!(
            tokenize("File created on 07/09/2017 at 15:58:57

[header]
satellites
time").collect::<Vec<_>>(),
            vec![
                Token::Line("File created on 07/09/2017 at 15:58:57"),
                Token::SectionHeader("header"),
                Token::Line("satellites"),
                Token::Line("time"),
            ]
        );
    }


    #[test]
    fn test_tokenize_from_reader() {
        let data = "\
File created on 07/09/2017 at 15:58:57

[header]
satellites
time
";
        let cursor = io::Cursor::new(data);

        assert_eq!(
            tokenize(cursor).collect::<Vec<_>>(),
            vec![
                Token::Line("File created on 07/09/2017 at 15:58:57"),
                Token::SectionHeader("header"),
                Token::Line("satellites"),
                Token::Line("time"),
            ]
        );
    }
}

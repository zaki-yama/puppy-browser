use crate::dom::{AttrMap, Element, Node};
use combine::error::ParseError;
use combine::parser::char::{char, letter, newline, space};
use combine::{between, many, many1, satisfy, Parser, Stream};

fn attribute<Input>() -> impl Parser<Input, Output = (String, String)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        many1::<String, _, _>(letter()), // まずは属性の名前を何文字か読む
        many::<String, _, _>(space().or(newline())), // 空白と空行を読み飛ばす
        char('='),                       // = を読む
        many::<String, _, _>(space().or(newline())), // 空白と改行を読み飛ばす
        between(
            char('"'),
            char('"'),
            many1::<String, _, _>(satisfy(|c: char| c != '"')),
        ),
    )
        .map(|v| (v.0, v.4)) // はじめに読んだ属性の名前と、最後に読んだ引用符の中の文字列を結果として返す
}

#[cfg(test)]
mod tests {
    use crate::dom::Text;

    use super::*;

    // parsing tests of attributes
    #[test]
    fn test_parse_attribute() {
        assert_eq!(
            attribute().parse("test=\"foobar\""),
            Ok((("test".to_string(), "foobar".to_string()), ""))
        );


        assert_eq!(
            attribute().parse("test = \"foobar\""),
            Ok((("test".to_string(), "foobar".to_string()), ""))
        );
    }
}

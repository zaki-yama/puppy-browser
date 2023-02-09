use crate::dom::{AttrMap, Element, Node, Text};
use combine::error::{ParseError, StreamError};
use combine::parser::char::{char, letter, newline, space};
use combine::{attempt, between, choice, many, many1, parser, satisfy, sep_by, Parser, Stream};

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

// `attributes` consumes `name1="value1" name2="value2" ... name="value"`
fn attributes<Input>() -> impl Parser<Input, Output = AttrMap>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    sep_by::<Vec<(String, String)>, _, _, _>(
        attribute(),
        many::<String, _, _>(space().or(newline())),
    )
    .map(|attrs: Vec<(String, String)>| {
        let m: AttrMap = attrs.into_iter().collect();
        m
    })
}

/// `open_tag` consumes `<tag_name attr_name="attr_value" ...>`
fn open_tag<Input>() -> impl Parser<Input, Output = (String, AttrMap)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let open_tag_name = many1::<String, _, _>(letter());
    let open_tag_content = (
        open_tag_name,
        many::<String, _, _>(space().or(newline())),
        attributes(),
    )
        .map(|v: (String, _, AttrMap)| (v.0, v.2));
    between(char('<'), char('>'), open_tag_content)
}

/// `close_tag` consumes `</tag_name>`
fn close_tag<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let close_tag_name = many1::<String, _, _>(letter());
    let close_tag_content = (char('/'), close_tag_name).map(|v| v.1);
    between(char('<'), char('>'), close_tag_content)
}

fn nodes_<Input>() -> impl Parser<Input, Output = Vec<Box<Node>>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    attempt(many(choice((attempt(element()), attempt(text())))))
}

parser! {
    fn nodes[Input]()(Input) -> Vec<Box<Node>>
    where [Input: Stream<Token = char>]
    {
        nodes_()
    }
}

fn text<Input>() -> impl Parser<Input, Output = Box<Node>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(satisfy(|c: char| c != '<')).map(|t| Text::new(t))
}

fn element<Input>() -> impl Parser<Input, Output = Box<Node>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (open_tag(), nodes(), close_tag()).and_then(
        |((open_tag_name, attributes), children, close_tag_name)| {
            if open_tag_name == close_tag_name {
                Ok(Element::new(open_tag_name, attributes, children))
            } else {
                Err(<Input::Error as combine::error::ParseError<
                    char,
                    Input::Range,
                    Input::Position,
                >>::StreamError::message_static_message(
                    "tag name of open tag and close tag mismatched",
                ))
            }
        },
    )
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

    #[test]
    fn test_parse_attributes() {
        let mut expected_map = AttrMap::new();
        expected_map.insert("test".to_string(), "foobar".to_string());
        expected_map.insert("abc".to_string(), "def".to_string());
        assert_eq!(
            attributes().parse("test=\"foobar\" abc=\"def\""),
            Ok((expected_map, ""))
        );

        assert_eq!(attributes().parse(""), Ok((AttrMap::new(), "")))
    }

    #[test]
    fn test_parse_open_tag() {
        {
            assert_eq!(
                open_tag().parse("<p>aaaa"),
                Ok((("p".to_string(), AttrMap::new()), "aaaa"))
            );
        }
    }

    #[test]
    fn test_parse_close_tag() {
        {
            assert_eq!(close_tag().parse("</p>"), Ok(("p".to_string(), "")));
        }
    }

    #[test]
    fn test_parse_element() {
        assert_eq!(
            element().parse("<p></p>"),
            Ok((Element::new("p".to_string(), AttrMap::new(), vec![]), ""))
        );

        assert_eq!(
            element().parse("<p>hello world</p>"),
            Ok((
                Element::new(
                    "p".to_string(),
                    AttrMap::new(),
                    vec![Text::new("hello world".to_string())]
                ),
                ""
            ))
        );

        assert_eq!(
            element().parse("<div><p>hello world</p></div>"),
            Ok((
                Element::new(
                    "div".to_string(),
                    AttrMap::new(),
                    vec![Element::new(
                        "p".to_string(),
                        AttrMap::new(),
                        vec![Text::new("hello world".to_string())]
                    )],
                ),
                ""
            ))
        );
    }
}

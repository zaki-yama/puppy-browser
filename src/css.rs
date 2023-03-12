use combine::{ParseError, Parser, Stream};

#[derive(Debug, PartialEq)]
struct Stylesheet {
    pub rules: Vec<Rule>,
}

impl Stylesheet {
    pub fn new(rules: Vec<Rule>) -> Self {
        Stylesheet { rules }
    }
}

#[derive(Debug, PartialEq)]

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

/// NOTE: This is not compliant to the standard for simplicity.
///
/// In the standard, *a selector* is *a chain* of one or more sequences of simple selectors separated by combinators,
/// where a sequence of simple selectors is a chain of simple selectors that are not separated by a combinator.
/// Hence `Selector` is in fact something like `Vec<Vec<SimpleSelector>>`.
pub type Selector = SimpleSelector;

#[derive(Debug, PartialEq)]
pub enum SimpleSelector {
    UniversalSelector,
    TypeSelector {
        tag_name: String,
    },
    AttributeSelector {
        tag_name: String,
        op: AttributeSelectorOp,
        attribute: String,
        value: String,
    },
    ClassSelector {
        class_name: String,
    },
    // TODO (enhancement): support multiple attribute selectors like `a[href=bar][ping=foo]`
    // TODO (enhancement): support more attribute selectors
}

#[derive(Debug, PartialEq)]
pub enum AttributeSelectorOp {
    Eq,      // =
    Contain, // ~=
}

/// `Declaration` represents a CSS declaration defined at [CSS Syntax Module Level 3](https://www.w3.org/TR/css-syntax-3/#declaration)
///
/// Declarations are further categorized into the followings:
/// - descriptors, which are mostly used in "at-rules" like `@foo (bar: piyo)` https://www.w3.org/Style/CSS/all-descriptors.en.html
/// - properties, which are mostly used in "qualified rules" like `.foo {bar: piyo}` https://www.w3.org/Style/CSS/all-descriptors.en.html
///
/// For simplicity, we handle two types of declarations together.
#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub value: CSSValue,
}

/// `CSSValue` represents some of *component value types* defined at [CSS Values and Units Module Level 3](https://www.w3.org/TR/css-values-3/#component-types).
#[derive(Debug, PartialEq)]
pub enum CSSValue {
    Keyword(String),
}

pub fn parse(raw: &str) -> Stylesheet {}

fn rules<Input>() -> impl Parser<Input, Output = Vec<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this");
}

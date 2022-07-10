use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::{eof, map};
use nom::error::VerboseError;
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;

use crate::node::{AndNode, ContradictionNode, IfThenNode, Node, NotNode, OrNode, PropositionNode};

type NodeResult<'l> = IResult<&'l [u8], Box<dyn Node>, VerboseError<&'l [u8]>>;

macro_rules! parse_separated_list {
    ($($c:expr),*) => {{
        map(
            tuple((
                parse_factor,
                alt(($(tag($c)),*)),
                separated_list1(alt(($(tag($c)),*)), parse_factor)
            )),
            |(first, _, mut vec)| {
                vec.insert(0, first);
                vec
            }
        )
    }};
}

pub fn parse_whole_term(term: &str) -> Option<Box<dyn Node>> {
    let term = term.replace(" ", "");
    let term_bytes = term.as_bytes();
    let (_, (node, _)) = tuple((parse_term, eof))(term_bytes).ok()?;

    Some(node)
}

pub fn parse_term(fragment: &[u8]) -> NodeResult {
    alt((parse_and, parse_or, parse_if_then, parse_factor))(fragment)
}

fn parse_and(fragment: &[u8]) -> NodeResult {
    let (fragment, factors) = parse_separated_list!("/\\", "∧", "&")(fragment)?;

    Ok((fragment, AndNode::new(factors)))
}

fn parse_or(fragment: &[u8]) -> NodeResult {
    let (fragment, factors) = parse_separated_list!("\\/", "∨", "||")(fragment)?;

    Ok((fragment, OrNode::new(factors)))
}

fn parse_if_then(fragment: &[u8]) -> NodeResult {
    let (fragment, (left, _, right)) =
        tuple((parse_factor, alt((tag("->"), tag("→"))), parse_factor))(fragment)?;

    Ok((fragment, IfThenNode::new(left, right)))
}

fn parse_factor(fragment: &[u8]) -> NodeResult {
    let (fragment, node) =
        alt((parse_prop, parse_contradiction, parse_paren, parse_not))(fragment)?;

    Ok((fragment, node))
}

fn parse_prop(fragment: &[u8]) -> NodeResult {
    let (fragment, c) = alpha1(fragment)?;
    let name = String::from_utf8_lossy(c).to_string();

    if name == "X" {
        Ok((fragment, ContradictionNode::new()))
    } else {
        Ok((fragment, PropositionNode::new(name)))
    }
}

fn parse_contradiction(fragment: &[u8]) -> NodeResult {
    let (fragment, _) = tag("⊥")(fragment)?;

    Ok((fragment, ContradictionNode::new()))
}

fn parse_paren(fragment: &[u8]) -> NodeResult {
    let (fragment, node) = delimited(tag("("), parse_term, tag(")"))(fragment)?;

    Ok((fragment, node))
}

fn parse_not(fragment: &[u8]) -> NodeResult {
    let (fragment, (_, prop)) =
        tuple((alt((tag("~"), tag("¬"), tag("!"))), parse_factor))(fragment)?;

    Ok((fragment, NotNode::new(prop)))
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_parse_term() {
        assert_eq!(
            parse_whole_term("P").map(|term| term.to_string()),
            Some("P".to_string())
        );

        assert_eq!(
            parse_whole_term("P/\\(~Q)").map(|term| term.to_string()),
            Some("(P ∧ ¬Q)".to_string())
        );
    }
}

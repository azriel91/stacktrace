use flat_string::FlatString;
use nom::{
    branch::alt, bytes::tag, combinator::opt, multi::many1, sequence::delimited, IResult, Parser,
};
use smallvec::SmallVec;

use crate::{GroupOrSegment, Segment};

/// A group of segments.
///
/// e.g. in `my::lib::<impl tachys::view::RenderHtml for (A,)>::{{closure}}`,
/// the groups are:
///
/// * `my`
/// * `lib`
/// * `<impl tachys::view::RenderHtml for (A,)>`
///     - `impl`
///     - `tachys::view::RenderHtml`
///     - `for`
///     - `(A,)`
///         - `A,`
/// * `{{closure}}`
///     - `{closure}`
///         - `closure`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
    /// The value between brackets (if brackets exist).
    ///
    /// e.g.
    ///
    /// * single segment: `my`
    /// * multiple groups: `impl tachys::view::RenderHtml for (A,)`
    pub value: SmallVec<[GroupOrSegment; 1]>,
    /// Any opening bracket for this group.
    pub bracket_open: Option<char>,
    /// Any closing bracket for this group.
    pub bracket_close: Option<char>,
    /// Any punctuation separator prior to the next group.
    pub separator: Option<FlatString<2>>,
}

impl Group {
    pub fn parse(input: &str) -> IResult<&str, Group> {
        alt((
            Self::parse_group(Some(('<', '>'))),
            Self::parse_group(Some(('[', ']'))),
            Self::parse_group(Some(('{', '}'))),
            Self::parse_group(Some(('(', ')'))),
            Self::parse_group(None),
        ))
        .parse(input)
    }

    fn parse_group(brackets: Option<(char, char)>) -> impl Fn(&str) -> IResult<&str, Group> {
        move |input| match brackets {
            Some((bracket_open, bracket_close)) => {
                let (input, group) = delimited(
                    nom::character::char(bracket_open),
                    Self::parse,
                    nom::character::char(bracket_close),
                )
                .parse(input)?;
                let bracket_open = Some(bracket_open);
                let bracket_close = Some(bracket_close);

                Ok((
                    input,
                    Group {
                        value,
                        bracket_open,
                        bracket_close,
                        separator,
                    },
                ))
            }
            None => {
                let (input, (value, separator)) = Self::parse_group_value(input)?;

                Ok((
                    input,
                    Group {
                        value,
                        bracket_open: None,
                        bracket_close: None,
                        separator,
                    },
                ))
            }
        }
    }

    fn parse_group_value(input: &str) -> IResult<&str, SmallVec<[GroupOrSegment; 1]>> {
        // We want to parse segments with the same separator as a group, and segments
        // with different separators as a different group. i.e. `a::b::C` is one group,
        // but `a::b C` are two groups.
        let (input, (inner_groups, separator)) =
            (Self::parse_group_value_single, opt(tag(" "))).parse(input)?;

        let value = match inner_groups.len() {
            1 => {
                let inner_group = inner_groups
                    .into_iter()
                    .next()
                    .expect("Expected first inner group to exist");
            }
            _ => {
                todo!()
            }
        };

        Ok((input, value))
    }

    fn parse_group_value_single(input: &str) -> IResult<&str, SmallVec<[GroupOrSegment; 1]>> {
        let (input, (segments, separator)) = (
            many1(Self::group_separator, Segment::parse),
            opt(Self::group_separator_or_space),
        )
            .parse(input)?;

        match segments.len() {
            1 => {
                let segment = segments
                    .into_iter()
                    .next()
                    .expect("Expected first segment to exist");
                let mut value = SmallVec::new();
                value.push(GroupOrSegment::Segment(segment));

                value
            }
            _ => {
                let segments_len = segments.len();
                let inner_group = {
                    let inner_group_value = segments.into_iter().fold(
                        SmallVec::with_capacity(segments_len),
                        |mut inner_value, segment| {
                            inner_value.push(GroupOrSegment::Segment(segment));
                            inner_value
                        },
                    );

                    Group {
                        value: inner_group_value,
                        bracket_open: None,
                        bracket_close: None,
                        separator: None,
                    }
                };
                let mut value = SmallVec::new();
                value.push(GroupOrSegment::Group(Box::new(inner_group)));

                value
            }
        }

        Ok((input, value))
    }

    fn group_separator(input: &str) -> IResult<&str, &str> {
        alt((tag("."), tag("::"))).parse(input)
    }

    fn group_separator_or_space(input: &str) -> IResult<&str, &str> {
        alt((tag("."), tag("::"), tag(" "))).parse(input)
    }
}

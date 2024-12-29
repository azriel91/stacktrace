use nom::{character::multispace0, multi::many0, IResult, Parser};

use crate::Group;

/// One line in the stack trace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    /// Any leading whitespace for this line.
    pub leading_whitespace: String,
    /// The groups of segments within this line.
    ///
    /// e.g. in `my::lib::{{closure}}`, the groups are:
    ///
    /// * `my::lib`
    /// * `{{closure}}`
    ///
    /// Each group has its own list of segments.
    pub groups: Vec<Group>,
}

impl Line {
    pub fn parse(input: &str) -> IResult<&str, Line> {
        let (input, leading_whitespace) = multispace0().parse(input)?;
        let (input, groups) = Self::parse_groups(input)?;

        let leading_whitespace = leading_whitespace.to_string();
        Ok((
            input,
            Line {
                leading_whitespace,
                groups,
            },
        ))
    }

    fn parse_groups(input: &str) -> IResult<&str, Vec<Group>> {
        many0(Group::parse).parse(input)
    }
}

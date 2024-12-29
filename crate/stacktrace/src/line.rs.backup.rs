use std::{iter::Peekable, str::Chars};

use flat_string::FlatString;
use smallvec::SmallVec;

use crate::{Group, GroupOrSegment, Segment};

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

impl<'s> From<&'s str> for Line {
    fn from(line_str: &'s str) -> Self {
        let first_non_whitespace_char_pos = line_str.find(|c| !char::is_whitespace(c));
        match first_non_whitespace_char_pos {
            Some(first_non_whitespace_char_pos) => {
                let (leading_whitespace, remainder) =
                    line_str.split_at(first_non_whitespace_char_pos);
                let leading_whitespace = leading_whitespace.to_string();
                let groups = parse_groups(remainder);

                Line {
                    leading_whitespace,
                    groups,
                }
            }
            None => {
                let leading_whitespace = String::new();
                let groups = parse_groups(line_str);
                Line {
                    leading_whitespace,
                    groups,
                }
            }
        }
    }
}

fn parse_groups(line_str: &str) -> Vec<Group> {
    let mut chars = line_str.chars().peekable();
    let mut groups = Vec::new();
    while chars.peek().is_some() {
        groups.push(parse_group(&mut chars));
    }

    groups
}

fn parse_group(chars: &mut Peekable<Chars>) -> ParseGroupResult {
    let mut parse_state = ParseState::Empty;

    while let Some(c) = chars.peek().copied() {
        match c {
            '<' | '(' | '[' | '{' => {
                parse_state = match parse_state {
                    ParseState::Empty => {
                        chars.next(); // advance character
                        ParseState::GroupWithBracket {
                            value: SmallVec::new(),
                            buffer: String::with_capacity(16),
                            bracket_open: c,
                        }
                    }
                    ParseState::GroupParsed { value } => {
                        return ParseGroupResult::Ok(Group {
                            value,
                            bracket_open: None,
                            bracket_close: None,
                            separator: None,
                        });
                    }
                    ParseState::GroupAndPartial { mut value, buffer } => {
                        value.push(GroupOrSegment::Segment(Segment {
                            text: buffer,
                            separator: None,
                        }));
                        return ParseGroupResult::Ok(Group {
                            value,
                            bracket_open: None,
                            bracket_close: None,
                            separator: None,
                        });
                    }
                    ParseState::GroupWithBracket {
                        mut value,
                        buffer,
                        bracket_open,
                    } => {
                        value.push(GroupOrSegment::Segment(Segment {
                            text: buffer,
                            separator: None,
                        }));

                        // note: don't advance character so that nested call reads the
                        // character as `bracket_open`.
                        match parse_group(chars) {
                            ParseGroupResult::Ok(nested_group) => {
                                value.push(GroupOrSegment::Group(Box::new(nested_group)))
                            }
                            ParseGroupResult::BracketCloseEncountered => {
                                // do nothing, the next loop will encounter the
                                // close bracket and close the group.
                            }
                        }

                        ParseState::GroupWithBracket {
                            value,
                            buffer: String::with_capacity(16),
                            bracket_open,
                        }
                    }
                };
            }
            '>' | ')' | ']' | '}' => match parse_state {
                ParseState::Empty => {
                    // This causes an infinite loop if there are no parent invocations that are in a
                    // different `ParseState`, and we retry.
                    //
                    // So we need to indicate a closing bracket.

                    return ParseGroupResult::BracketCloseEncountered;
                }
                ParseState::GroupParsed { value } => {
                    return ParseGroupResult::Ok(Group {
                        value,
                        bracket_open: None,
                        bracket_close: None,
                        separator: None,
                    });
                }
                ParseState::GroupAndPartial { mut value, buffer } => {
                    value.push(GroupOrSegment::Segment(Segment {
                        text: buffer,
                        separator: None,
                    }));

                    return ParseGroupResult::Ok(Group {
                        value,
                        bracket_open: None,
                        bracket_close: None,
                        separator: None,
                    });
                }
                ParseState::GroupWithBracket {
                    mut value,
                    buffer,
                    bracket_open,
                } => {
                    // TODO: should we ensure the closing bracket matches the opening bracket?

                    value.push(GroupOrSegment::Segment(Segment {
                        text: buffer,
                        separator: None,
                    }));

                    let bracket_open = Some(bracket_open);
                    let bracket_close = Some(c);

                    // advance character so outer call does not re-read the closing bracket.
                    chars.next();

                    return ParseGroupResult::Ok(Group {
                        value,
                        bracket_open,
                        bracket_close,
                        separator: None,
                    });
                }
            },
            // separators
            ' ' | ':' | '.' | '@' => {
                chars.next(); // advance character
                parse_state = match parse_state {
                    ParseState::Empty => {
                        let mut buffer = String::with_capacity(16);
                        buffer.push(c);
                        ParseState::GroupAndPartial {
                            value: SmallVec::new(),
                            buffer,
                        }
                    }
                    ParseState::GroupParsed { value } => {
                        let mut buffer = String::with_capacity(16);
                        buffer.push(c);

                        ParseState::GroupAndPartial { value, buffer }
                    }
                    ParseState::GroupAndPartial { mut value, buffer } => {
                        let buffer = buffer_or_value_append(buffer, &mut value, c)
                            .unwrap_or_else(|| String::with_capacity(16));

                        ParseState::GroupAndPartial { value, buffer }
                    }
                    ParseState::GroupWithBracket {
                        mut value,
                        buffer,
                        bracket_open,
                    } => {
                        let buffer = buffer_or_value_append(buffer, &mut value, c)
                            .unwrap_or_else(|| String::with_capacity(16));

                        ParseState::GroupWithBracket {
                            value,
                            buffer,
                            bracket_open,
                        }
                    }
                };
            }

            // everything else we consider part of a segment,
            // this includes commas
            _ => {
                parse_state = match parse_state {
                    ParseState::Empty => {
                        let mut buffer = String::with_capacity(16);
                        buffer.push(c);
                        ParseState::GroupAndPartial {
                            value: SmallVec::new(),
                            buffer,
                        }
                    }
                    ParseState::GroupParsed { value } => {
                        let mut buffer = String::with_capacity(16);
                        buffer.push(c);

                        ParseState::GroupAndPartial { value, buffer }
                    }
                    ParseState::GroupAndPartial { mut value, buffer } => {
                        let buffer = buffer_or_value_append(buffer, &mut value, c)
                            .unwrap_or_else(|| String::with_capacity(16));

                        ParseState::GroupAndPartial { value, buffer }
                    }
                    ParseState::GroupWithBracket {
                        mut value,
                        buffer,
                        bracket_open,
                    } => {
                        let buffer = buffer_or_value_append(buffer, &mut value, c)
                            .unwrap_or_else(|| String::with_capacity(16));

                        ParseState::GroupWithBracket {
                            value,
                            buffer,
                            bracket_open,
                        }
                    }
                };
            }
        }
    }

    match parse_state {
        ParseState::Empty => todo!(),
        ParseState::GroupParsed { value } => todo!(),
        ParseState::GroupAndPartial { value, buffer } => todo!(),
        ParseState::GroupWithBracket {
            value,
            buffer,
            bracket_open,
        } => todo!(),
    }
}

/// Appends the character to the buffer if it is the same spacer character, or
/// pushes the current buffer as a new segment otherwise.
fn buffer_or_value_append(
    mut buffer: String,
    value: &mut SmallVec<[GroupOrSegment; 1]>,
    c: char,
) -> Option<String> {
    let buffer_chars_all_eq_c = buffer.chars().all(|buffer_c| buffer_c == c);
    match buffer_chars_all_eq_c {
        // If the buffer only contains the same separator character, we want to
        // append the current character to it.
        true => {
            buffer.push(c);
            Some(buffer)
        }

        // Otherwise we treat this as a new Segment Group.
        false => {
            let separator = {
                let mut separator = FlatString::new();
                separator.push(c);
                Some(separator)
            };
            value.push(GroupOrSegment::Segment(Segment {
                text: buffer,
                separator,
            }));
            None
        }
    }
}

/// Parse state for one `Group`.
#[derive(Clone, Debug, PartialEq, Eq)]
enum ParseState {
    Empty,
    GroupParsed {
        /// Previously parsed groups.
        value: SmallVec<[GroupOrSegment; 1]>,
    },
    GroupPartial {
        /// Segment that is being parsed.
        buffer: String,
    },
    GroupAndPartial {
        /// Previously parsed groups.
        value: SmallVec<[GroupOrSegment; 1]>,
        /// Segment that is being parsed.
        buffer: String,
    },
    GroupWithBracket {
        /// Previously parsed groups.
        value: SmallVec<[GroupOrSegment; 1]>,
        /// Segment that is being parsed.
        buffer: String,
        /// Bracket that opened this group.
        bracket_open: char,
    },
}

#[derive(Debug)]
enum ParseGroupResult {
    Ok(Group),
    BracketCloseEncountered,
}

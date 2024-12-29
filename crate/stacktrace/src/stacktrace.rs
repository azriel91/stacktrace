use std::{cmp::Ordering, iter::Peekable, str::Lines};

use crate::Section;

/// Parses a stack trace string into a structured stack trace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stacktrace {
    pub sections: Vec<Section>,
}

impl Stacktrace {
    fn parse(
        lines: &mut Peekable<Lines>,
        previous_line: Option<&str>,
        previous_line_slice_common_len: Option<usize>,
    ) -> Vec<Section> {
        let mut sections = Vec::new();

        let line = lines.peek();
        match line {
            Some(line) => {
                let slice_common_with_ancestors = previous_line
                    .and_then(|previous_line| {
                        line.chars()
                            .zip(previous_line.chars())
                            .take_while(|(line_char, previous_line_char)| {
                                line_char == previous_line_char
                            })
                            .enumerate()
                            .last()
                            .map(|(byte_index, _line_and_previous_line_char)| byte_index)
                            .map(|slice_common_end_index| {
                                line[..slice_common_end_index].to_string()
                            })
                    })
                    .unwrap_or_default();

                // if the slice common with ancestors is shorter than or equal to the previous
                // line's slice common length, then this line should be a subsection of the
                // parent section
                let should_return_early = previous_line_slice_common_len
                    .as_ref()
                    .map(|previous_line_slice_common_len| {
                        slice_common_with_ancestors
                            .len()
                            .cmp(previous_line_slice_common_len)
                    })
                    .map(|comparison| match comparison {
                        Ordering::Less | Ordering::Equal => true,
                        Ordering::Greater => false,
                    })
                    .unwrap_or(false);
                match should_return_early {
                    true => return sections,
                    false => {}
                }

                let slice_remainder = match slice_common_with_ancestors.len() == line.len() {
                    true => String::new(),
                    false => line[slice_common_with_ancestors.len()..].to_string(),
                };

                let current_line = line.to_string();

                // consume the line because we are starting a new `Section`.
                lines.next();

                let child_sections = Self::parse(
                    lines,
                    Some(current_line.as_str()),
                    Some(slice_common_with_ancestors.len()),
                );

                let section = Section {
                    slice_common_with_ancestors,
                    slice_remainder,
                    child_sections,
                };
                sections.push(section);
            }
            None => {
                // advance this line so we don't double process it.
                lines.next();
            }
        }

        sections
    }
}

impl<'s> From<&'s str> for Stacktrace {
    fn from(s: &'s str) -> Self {
        let mut lines = s.lines().peekable();
        let sections = Stacktrace::parse(&mut lines, None, None);

        Self { sections }
    }
}

use std::{cmp::Ordering, iter::Peekable, str::Lines};

use crate::Section;

/// Parses a stack trace string into a structured stack trace.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stacktrace {
    pub sections: Vec<Section>,
}

impl Stacktrace {
    fn parse(
        lines: &mut Peekable<Lines>,
        next_id: &mut u32,
        previous_section_info: Option<PreviousSectionInfo<'_>>,
    ) -> Vec<Section> {
        let mut sections = Vec::new();

        while let Some(line) = lines.peek() {
            let slice_common_with_ancestors =
                Self::parse_slice_common_with_ancestors(previous_section_info, line);

            // if the slice common with ancestors is shorter than or equal to the previous
            // line's slice common length, then this line should be a subsection of the
            // parent section
            let should_return_early = Self::line_is_better_suited_as_child_section_of_parent(
                previous_section_info,
                &slice_common_with_ancestors,
            );
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

            let section_id = *next_id;
            *next_id += 1;

            let child_sections = Self::parse(
                lines,
                next_id,
                Some(PreviousSectionInfo {
                    previous_line: current_line.as_str(),
                    slice_common_len: slice_common_with_ancestors.len(),
                }),
            );

            let section = Section {
                id: section_id,
                slice_common_with_ancestors,
                slice_remainder,
                child_sections,
            };
            sections.push(section);
        }

        sections
    }

    fn parse_slice_common_with_ancestors(
        previous_section_info: Option<PreviousSectionInfo<'_>>,
        line: &&str,
    ) -> String {
        let slice_common_with_ancestors = previous_section_info
            .map(PreviousSectionInfo::previous_line)
            .and_then(|previous_line| {
                line.chars()
                    .zip(previous_line.chars())
                    .take_while(|(line_char, previous_line_char)| line_char == previous_line_char)
                    .enumerate()
                    .last()
                    .map(|(commona_char_index, _line_and_previous_line_char)| commona_char_index)
                    .and_then(|common_char_index| {
                        Self::beginning_of_closest_separator(line, common_char_index)
                    })
                    .map(|slice_common_end_index| line[..slice_common_end_index].to_string())
            })
            .unwrap_or_default();
        slice_common_with_ancestors
    }

    fn beginning_of_closest_separator(line: &str, common_char_index: usize) -> Option<usize> {
        (&line[..=common_char_index])
            .rfind(Self::is_separator)
            .and_then(|separator_index| {
                let line_until_separator_end = &line[..separator_index];
                line_until_separator_end
                    .rfind(Self::is_word_character)
                    // `+ 1` because we want the index after the last word charater.
                    .map(|previous_word_index| previous_word_index + 1)
            })
    }

    fn is_separator(c: char) -> bool {
        !Self::is_word_character(c)
    }

    fn is_word_character(c: char) -> bool {
        char::is_alphanumeric(c) || c == '_'
    }

    fn line_is_better_suited_as_child_section_of_parent(
        previous_section_info: Option<PreviousSectionInfo<'_>>,
        slice_common_with_ancestors: &String,
    ) -> bool {
        previous_section_info
            .map(PreviousSectionInfo::slice_common_len)
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
            .unwrap_or(false)
    }
}

impl<'s> From<&'s str> for Stacktrace {
    fn from(s: &'s str) -> Self {
        let mut lines = s.lines().peekable();
        let sections = Stacktrace::parse(&mut lines, &mut 0, None);

        Self { sections }
    }
}

#[derive(Clone, Copy, Debug)]
struct PreviousSectionInfo<'s> {
    previous_line: &'s str,
    slice_common_len: usize,
}

impl<'s> PreviousSectionInfo<'s> {
    fn previous_line(self) -> &'s str {
        self.previous_line
    }

    fn slice_common_len(self) -> usize {
        self.slice_common_len
    }
}

#[cfg(test)]
mod tests {
    use crate::Section;

    use super::Stacktrace;

    #[test]
    fn parses_multiple_section_stacktrace_simple() {
        let stacktrace = Stacktrace::from(
            "\
            a::b::Class.method_one\n\
            a::b::Class.method_two\n\
            ",
        );

        assert_eq!(
            Stacktrace {
                sections: vec![Section {
                    id: 0,
                    slice_common_with_ancestors: String::new(),
                    slice_remainder: String::from("a::b::Class.method_one"),
                    child_sections: vec![Section {
                        id: 1,
                        slice_common_with_ancestors: String::from("a::b::Class"),
                        slice_remainder: String::from(".method_two"),
                        child_sections: Vec::new()
                    }]
                }]
            },
            stacktrace
        )
    }

    #[test]
    fn parses_multiple_section_stacktrace_wasm() {
        let stacktrace = Stacktrace::from(
            "\
            __wbg_get_imports/imports.wbg.__wbg_new_abda76e883ba8a5f@http://127.0.0.1:7890/pkg/dot_ix.js:489:13\n\
            dot_ix_playground.wasm.__wbg_new_abda76e883ba8a5f externref shim@http://127.0.0.1:7890/pkg/dot_ix.wasm:wasm-function[25993]:0x6bb546\n\
            dot_ix_playground.wasm.console_error_panic_hook::Error::new::h8adb78d6eba1ab93@http://127.0.0.1:7890/pkg/dot_ix.wasm:wasm-function[16925]:0x636d40\n\
            ",
        );

        assert_eq!(
            Stacktrace {
                sections: vec![
                    Section {
                        id: 0,
                        slice_common_with_ancestors: String::new(),
                        slice_remainder: String::from("__wbg_get_imports/imports.wbg.__wbg_new_abda76e883ba8a5f@http://127.0.0.1:7890/pkg/dot_ix.js:489:13"),
                        child_sections: vec![]
                    },
                    Section {
                        id: 1,
                        slice_common_with_ancestors: String::new(),
                        slice_remainder: String::from("dot_ix_playground.wasm.__wbg_new_abda76e883ba8a5f externref shim@http://127.0.0.1:7890/pkg/dot_ix.wasm:wasm-function[25993]:0x6bb546"),
                        child_sections: vec![Section {
                            id: 2,
                            slice_common_with_ancestors: String::from("dot_ix_playground.wasm"),
                            slice_remainder: String::from(".console_error_panic_hook::Error::new::h8adb78d6eba1ab93@http://127.0.0.1:7890/pkg/dot_ix.wasm:wasm-function[16925]:0x636d40"),
                            child_sections: Vec::new()
                        }]
                    },
                ]
            },
            stacktrace
        )
    }
}

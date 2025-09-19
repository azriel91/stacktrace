use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{
    file::FilePathAndLine,
    java::{JavaClassNameQualified, JavaMethodName},
    logs_parser::Rule,
};

/// The first line of a Java stacktrace.
///
/// i.e. the `Exception in thread "main" java.lang.IllegalArgumentException:
/// foo` in the following:
///
/// ```java
/// Exception in thread "main" java.lang.IllegalArgumentException: foo
///     at com.example.stacktrace.Example.fail(Example.java:11)
///     at java.lang.Thread.run(Thread.java:750)
/// Caused by: com.example.stacktrace.Example$Exception: bar
///     at com.example.stacktrace.Example.fail(Example.java:12)
/// ... 2 more
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaStacktraceFrame<'s> {
    /// The full line of text of the frame.
    pub full_text: Cow<'s, str>,
    /// The `at` text.
    pub at: Cow<'s, str>,
    /// The `Exception in thread ".."` text, if any.
    pub class_name_qualified: JavaClassNameQualified<'s>,
    /// The `.` separator between the class name ane method name.
    pub dot: Cow<'s, str>,
    /// The `com.example.stacktrace.Example$Exception` text.
    pub method_name: JavaMethodName<'s>,
    /// The `(` between the method name and the parameters.
    pub parenthesis_open: Cow<'s, str>,
    /// The `Example.java:11` text inside the parentheses.
    pub file_path_and_line: FilePathAndLine<'s>,
    /// The `)` between the method name and the parameters.
    pub parenthesis_close: Cow<'s, str>,
}

impl<'s> From<Pair<'s, Rule>> for JavaStacktraceFrame<'s> {
    fn from(java_stacktrace_frame_pair: Pair<'s, Rule>) -> Self {
        let full_text = Cow::Borrowed(java_stacktrace_frame_pair.as_str());
        let (class_name_qualified, method_name, file_path_and_line) =
            java_stacktrace_frame_pair.into_inner().fold(
                (None, None, None),
                |(mut class_name_qualified, mut method_name, mut file_path_and_line),
                 java_stacktrace_frame_pair_inner| {
                    match java_stacktrace_frame_pair_inner.as_rule() {
                        Rule::JavaClassNameQualified => {
                            let class_name_qualified_pair = java_stacktrace_frame_pair_inner;
                            class_name_qualified =
                                Some(JavaClassNameQualified::from(class_name_qualified_pair));

                            (class_name_qualified, method_name, file_path_and_line)
                        }
                        Rule::JavaMethodName => {
                            let method_name_pair = java_stacktrace_frame_pair_inner;
                            method_name = Some(JavaMethodName::from(method_name_pair));

                            (class_name_qualified, method_name, file_path_and_line)
                        }
                        Rule::FilePathAndLine => {
                            let file_path_and_line_pair = java_stacktrace_frame_pair_inner;
                            file_path_and_line =
                                Some(FilePathAndLine::from(file_path_and_line_pair));

                            (class_name_qualified, method_name, file_path_and_line)
                        }
                        _ => unreachable!(),
                    }
                },
            );

        let class_name_qualified = class_name_qualified
            .expect("Expected `JavaClassNameQualified` to exist after parsing.");
        let method_name = method_name.expect("Expected `JavaMethodName` to exist after parsing.");
        let file_path_and_line =
            file_path_and_line.expect("Expected `FilePathAndLine` to exist after parsing.");

        Self {
            full_text,
            at: Cow::Borrowed("at"),
            class_name_qualified,
            dot: Cow::Borrowed("."),
            method_name,
            parenthesis_open: Cow::Borrowed("("),
            file_path_and_line,
            parenthesis_close: Cow::Borrowed(")"),
        }
    }
}

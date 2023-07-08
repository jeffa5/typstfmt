use crate::config::Config;

/// A context object used to store state while formatting.
#[derive(Default)]
pub struct Writer {
    /// The current value written.
    value: String,
    /// The config to use for formatting the text.
    config: Config,
    /// The current indentation level, in spaces.
    indent_level: usize,
}

impl Writer {
    pub fn new(style: Config) -> Self {
        Self {
            value: String::new(),
            config: style,
            indent_level: 0,
        }
    }

    /// Appends the amount of spaces defined by the style.
    pub fn indent(&mut self) -> &mut Self {
        self.push(&" ".repeat(self.indent_level));
        self
    }

    /// Appends the given text to the buffer.
    pub fn push(&mut self, s: &str) -> &mut Self {
        self.value.push_str(s);
        self
    }

    /// Appends a newline character to the buffer, followed by
    /// the current indentation level in spaces.
    pub fn newline_with_indent(&mut self) -> &mut Self {
        self.newline().indent();
        self
    }

    /// Appends a newline character to the buffer.
    pub fn newline(&mut self) -> &mut Self {
        self.push("\n");
        self
    }

    /// Increases the current indentation level by the amount specified in the style.
    pub fn inc_indent(&mut self) -> &mut Self {
        self.indent_level = self.indent_level.saturating_add(self.config.indent);
        self
    }

    /// Decreases the current indentation level by the amount specified in the style.
    pub fn dec_indent(&mut self) -> &mut Self {
        self.indent_level = self.indent_level.saturating_sub(self.config.indent);
        self
    }

    /// Get the written value.
    pub fn finish(self) -> String {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_persistent() {
        let mut writer = Writer::default();
        writer.inc_indent();
        writer.newline_with_indent();
        writer.push("Hello, World!");
        similar_asserts::assert_eq!(writer.finish(), "\n    Hello, World!");
    }

    #[test]
    fn complex() {
        let mut writer = Writer::default();
        let indent = writer.config.indent;
        writer
            .push("f(")
            .inc_indent()
            .newline_with_indent()
            .push("a,")
            .newline_with_indent()
            .push("b")
            .dec_indent()
            .newline_with_indent()
            .push(")");
        similar_asserts::assert_eq!(
            writer.finish(),
            format!("f(\n{0}a,\n{0}b\n)", " ".repeat(indent))
        );
    }

    #[test]
    fn indent_change() {
        let mut writer = Writer::default();
        let indent_style = writer.config.indent;
        similar_asserts::assert_eq!(writer.indent_level, 0);
        writer.inc_indent();
        similar_asserts::assert_eq!(writer.indent_level, indent_style);
        writer.inc_indent();
        similar_asserts::assert_eq!(writer.indent_level, 2 * indent_style);
        writer.dec_indent();
        similar_asserts::assert_eq!(writer.indent_level, indent_style);
        writer.dec_indent();
        similar_asserts::assert_eq!(writer.indent_level, 0);
        writer.dec_indent();
        similar_asserts::assert_eq!(writer.indent_level, 0);
    }
}

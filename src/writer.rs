use crate::config::Config;
use tracing::debug;

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
    /// Create a new writer with the given config.
    pub fn new(config: Config) -> Self {
        Self {
            value: String::new(),
            config,
            indent_level: 0,
        }
    }

    /// Push the current indentation amount.
    pub fn current_indent(&self) -> String {
        " ".repeat(self.indent_level)
    }

    /// Push the current indentation amount.
    pub fn indent(&mut self) -> &mut Self {
        self.push(&self.current_indent());
        self
    }

    /// Appends the given text to the buffer.
    pub fn push(&mut self, s: &str) -> &mut Self {
        debug!(?s, "push");
        self.value.push_str(s);
        self
    }

    pub fn parbreak(&mut self) -> &mut Self {
        if !self.value.ends_with("\n") {
            self.newline();
        }
        self.newline().indent();
        self
    }

    pub fn space(&mut self) -> &mut Self {
        if self.value.ends_with('\n') {
            // never push a space after a newline, that should be handled by `newline_with_indent`
            return self;
        }
        self.push(" ")
    }

    /// Appends a newline character to the buffer, followed by
    /// the current indentation level in spaces.
    pub fn newline_with_indent(&mut self) -> &mut Self {
        if self
            .value
            .ends_with(&format!("\n{}", self.current_indent()))
        {
            // prevent double newlines
            return self;
        }
        self.newline().indent();
        self
    }

    /// Appends a newline character to the buffer.
    fn newline(&mut self) -> &mut Self {
        self.push("\n");
        self
    }

    /// Increases the current indentation level by the amount specified in the style.
    pub fn inc_indent(&mut self) -> &mut Self {
        debug!("inc_indent");
        self.indent_level = self.indent_level.saturating_add(self.config.indent);
        self
    }

    /// Decreases the current indentation level by the amount specified in the style.
    pub fn dec_indent(&mut self) -> &mut Self {
        debug!("dec_indent");
        self.indent_level = self.indent_level.saturating_sub(self.config.indent);
        self
    }

    pub fn open_grouping(&mut self, text: &str) -> &mut Self {
        debug!(?text, "open grouping");
        self.push(text).inc_indent();
        self
    }

    pub fn close_grouping(&mut self, text: &str) -> &mut Self {
        debug!(?text, "close grouping");
        // remove the previous indent if there was one
        if self.value.ends_with(&self.current_indent()) {
            for _ in 0..self.config.indent {
                self.value.remove(self.value.len() - 1);
            }
        }
        self.dec_indent();
        self.push(text);
        self
    }

    /// Get the written value.
    pub fn finish(self) -> String {
        self.value
    }

    pub fn config(&self) -> &Config {
        &self.config
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
        similar_asserts::assert_eq!(writer.finish(), "\n  Hello, World!");
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

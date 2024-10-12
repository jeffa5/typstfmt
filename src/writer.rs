use crate::config::Config;
use tracing::debug;

/// A context object used to store state while formatting.
#[derive(Default)]
pub struct Writer {
    /// The current value written.
    value: String,
    /// The config to use for formatting the text.
    config: Config,

    line: String,
    /// The current indentation level, in spaces.
    current_indent_level: usize,
    next_indent_level: usize,
    incremented_indents_in_line: u32,
}

impl Writer {
    /// Create a new writer with the given config.
    pub fn new(config: Config) -> Self {
        Self {
            value: String::new(),
            config,
            line: String::new(),
            current_indent_level: 0,
            next_indent_level: 0,
            incremented_indents_in_line: 0,
        }
    }

    fn flush_line(&mut self) {
        if !self.line.is_empty() {
            let line = format!(
                "{}{}",
                self.current_indent(),
                std::mem::take(&mut self.line)
            );
            debug!(?line, "flushing line");
            self.value.push_str(&line);
        } else {
            debug!("flushing empty line");
        }
        self.value.push('\n');
        self.current_indent_level = self.next_indent_level;
        self.incremented_indents_in_line = 0;
    }

    /// Push the current indentation amount.
    fn current_indent(&self) -> String {
        " ".repeat(self.current_indent_level)
    }

    /// Appends the given text to the buffer.
    pub fn push(&mut self, s: &str) -> &mut Self {
        debug!(?s, "push");
        for c in s.chars() {
            if c == '\n' {
                self.flush_line();
            } else {
                self.line.push(c);
            }
        }
        self
    }

    pub fn parbreak(&mut self) -> &mut Self {
        if !self.line.is_empty() {
            self.newline();
        }
        self.newline();
        self
    }

    pub fn space(&mut self) -> &mut Self {
        if self.line.is_empty() {
            // never push a space after a newline
            return self;
        }
        self.push(" ")
    }

    /// Appends a newline character to the buffer.
    pub fn newline(&mut self) -> &mut Self {
        self.push("\n");
        self
    }

    /// Increases the current indentation level by the amount specified in the style.
    pub fn inc_indent(&mut self) -> &mut Self {
        debug!("inc_indent");
        if self.incremented_indents_in_line == 0 {
            self.next_indent_level = self.next_indent_level.saturating_add(self.config.indent);
        }
        self.incremented_indents_in_line += 1;
        self
    }

    /// Decreases the current indentation level by the amount specified in the style.
    pub fn dec_indent(&mut self) -> &mut Self {
        debug!("dec_indent");
        self.incremented_indents_in_line = self.incremented_indents_in_line.saturating_sub(1);
        if self.incremented_indents_in_line == 0 {
            self.next_indent_level = self.next_indent_level.saturating_sub(self.config.indent);
            self.current_indent_level = self.next_indent_level;
        }
        self
    }

    pub fn open_grouping(&mut self, text: &str) -> &mut Self {
        debug!(?text, "open grouping");
        self.push(text).inc_indent();
        self
    }

    pub fn close_grouping(&mut self, text: &str) -> &mut Self {
        debug!(?text, "close grouping");
        self.dec_indent();
        self.push(text);
        self
    }

    /// Get the written value.
    pub fn finish(mut self) -> String {
        if !self.line.is_empty() {
            self.flush_line();
        }
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
        writer.newline();
        writer.push("Hello, World!");
        similar_asserts::assert_eq!(writer.finish(), "\n  Hello, World!\n");
    }

    #[test]
    fn complex() {
        let mut writer = Writer::default();
        let indent = writer.config.indent;
        writer
            .push("f(")
            .inc_indent()
            .newline()
            .push("a,")
            .newline()
            .push("b,")
            .newline()
            .dec_indent()
            .push(")");
        similar_asserts::assert_eq!(
            writer.finish(),
            format!("f(\n{0}a,\n{0}b,\n)\n", " ".repeat(indent))
        );
    }
}

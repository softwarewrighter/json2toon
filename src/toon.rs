/// TOON (Token-Oriented-Object-Notation) writer
///
/// Converts JSON structures to TOON format using flattened key-value pairs.
/// - Objects are flattened using dot notation
/// - Arrays use indexed notation (e.g., items.0, items.1)
/// - Strings are quoted
/// - Numbers, booleans, and null are unquoted
pub struct ToonWriter {
    buffer: String,
}

impl ToonWriter {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn write_string(&mut self, key: &str, value: &str) {
        let escaped = escape_string(value);
        self.buffer
            .push_str(&format!("{}=\"{}\"\n", key, escaped));
    }

    pub fn write_number(&mut self, key: &str, value: f64) {
        // Format number without unnecessary decimals
        if value.fract() == 0.0 && value.abs() < 1e15 {
            self.buffer.push_str(&format!("{}={}\n", key, value as i64));
        } else {
            self.buffer.push_str(&format!("{}={}\n", key, value));
        }
    }

    pub fn write_bool(&mut self, key: &str, value: bool) {
        self.buffer.push_str(&format!("{}={}\n", key, value));
    }

    pub fn write_null(&mut self, key: &str) {
        self.buffer.push_str(&format!("{}=null\n", key));
    }

    pub fn finish(self) -> String {
        self.buffer
    }
}

impl Default for ToonWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// Escape special characters in strings for TOON format
fn escape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(ch),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_string() {
        let mut writer = ToonWriter::new();
        writer.write_string("name", "Alice");
        assert_eq!(writer.finish(), "name=\"Alice\"\n");
    }

    #[test]
    fn test_write_string_with_escapes() {
        let mut writer = ToonWriter::new();
        writer.write_string("text", "Hello \"World\"\nNew line");
        assert_eq!(writer.finish(), "text=\"Hello \\\"World\\\"\\nNew line\"\n");
    }

    #[test]
    fn test_write_integer() {
        let mut writer = ToonWriter::new();
        writer.write_number("age", 42.0);
        assert_eq!(writer.finish(), "age=42\n");
    }

    #[test]
    fn test_write_float() {
        let mut writer = ToonWriter::new();
        writer.write_number("score", 98.5);
        assert_eq!(writer.finish(), "score=98.5\n");
    }

    #[test]
    fn test_write_bool() {
        let mut writer = ToonWriter::new();
        writer.write_bool("active", true);
        assert_eq!(writer.finish(), "active=true\n");
    }

    #[test]
    fn test_write_null() {
        let mut writer = ToonWriter::new();
        writer.write_null("optional");
        assert_eq!(writer.finish(), "optional=null\n");
    }

    #[test]
    fn test_escape_string() {
        assert_eq!(escape_string("simple"), "simple");
        assert_eq!(escape_string("with \"quotes\""), "with \\\"quotes\\\"");
        assert_eq!(escape_string("with\\backslash"), "with\\\\backslash");
        assert_eq!(escape_string("line\nbreak"), "line\\nbreak");
        assert_eq!(escape_string("tab\there"), "tab\\there");
    }

    #[test]
    fn test_multiple_writes() {
        let mut writer = ToonWriter::new();
        writer.write_string("name", "Bob");
        writer.write_number("age", 30.0);
        writer.write_bool("active", false);
        writer.write_null("middle");

        let result = writer.finish();
        assert!(result.contains("name=\"Bob\"\n"));
        assert!(result.contains("age=30\n"));
        assert!(result.contains("active=false\n"));
        assert!(result.contains("middle=null\n"));
    }
}

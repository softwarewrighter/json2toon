use crate::toon::ToonWriter;
use anyhow::{Context, Result};
use serde_json::Value;

pub struct Converter {
    verbose: bool,
}

impl Converter {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    /// Convert JSON string to TOON format
    pub fn convert(&self, json: &str) -> Result<String> {
        if self.verbose {
            println!("[INFO] Parsing JSON...");
        }

        let value: Value = serde_json::from_str(json)
            .context("Failed to parse JSON")?;

        if self.verbose {
            println!("[INFO] JSON parsed successfully");
            println!("[INFO] Converting to TOON format...");
        }

        let mut writer = ToonWriter::new();
        self.convert_value(&mut writer, "", &value)?;

        if self.verbose {
            println!("[INFO] Conversion complete");
        }

        Ok(writer.finish())
    }

    /// Recursively convert a JSON value to TOON format
    fn convert_value(&self, writer: &mut ToonWriter, prefix: &str, value: &Value) -> Result<()> {
        match value {
            Value::Null => {
                writer.write_null(prefix);
            }
            Value::Bool(b) => {
                writer.write_bool(prefix, *b);
            }
            Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    writer.write_number(prefix, f);
                } else {
                    anyhow::bail!("Invalid number: {}", n);
                }
            }
            Value::String(s) => {
                writer.write_string(prefix, s);
            }
            Value::Array(arr) => {
                if arr.is_empty() {
                    // Represent empty array with a special marker
                    writer.write_string(prefix, "[]");
                } else {
                    for (i, item) in arr.iter().enumerate() {
                        let key = if prefix.is_empty() {
                            format!("{}", i)
                        } else {
                            format!("{}.{}", prefix, i)
                        };
                        self.convert_value(writer, &key, item)?;
                    }
                }
            }
            Value::Object(obj) => {
                if obj.is_empty() {
                    // Represent empty object with a special marker
                    writer.write_string(prefix, "{}");
                } else {
                    for (key, val) in obj.iter() {
                        let full_key = if prefix.is_empty() {
                            key.clone()
                        } else {
                            format!("{}.{}", prefix, key)
                        };
                        self.convert_value(writer, &full_key, val)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Estimate the size of the TOON output
    pub fn estimate_size(&self, json: &str) -> Result<usize> {
        let value: Value = serde_json::from_str(json)
            .context("Failed to parse JSON")?;
        Ok(self.estimate_value_size(&value, ""))
    }

    fn estimate_value_size(&self, value: &Value, prefix: &str) -> usize {
        match value {
            Value::Null => prefix.len() + 6, // "key=null\n"
            Value::Bool(_) => prefix.len() + 7, // "key=false\n" (worst case)
            Value::Number(_) => prefix.len() + 25, // Allow for large numbers
            Value::String(s) => prefix.len() + s.len() + 4, // "key=\"val\"\n"
            Value::Array(arr) => {
                if arr.is_empty() {
                    prefix.len() + 5 // "key=[]\n"
                } else {
                    arr.iter()
                        .enumerate()
                        .map(|(i, item)| {
                            let key = if prefix.is_empty() {
                                format!("{}", i)
                            } else {
                                format!("{}.{}", prefix, i)
                            };
                            self.estimate_value_size(item, &key)
                        })
                        .sum()
                }
            }
            Value::Object(obj) => {
                if obj.is_empty() {
                    prefix.len() + 5 // "key={}\n"
                } else {
                    obj.iter()
                        .map(|(key, val)| {
                            let full_key = if prefix.is_empty() {
                                key.clone()
                            } else {
                                format!("{}.{}", prefix, key)
                            };
                            self.estimate_value_size(val, &full_key)
                        })
                        .sum()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_simple_types() {
        let converter = Converter::new(false);

        let json = r#"{"name": "Alice", "age": 30, "active": true, "middle": null}"#;
        let toon = converter.convert(json).unwrap();

        assert!(toon.contains("name=\"Alice\"\n"));
        assert!(toon.contains("age=30\n"));
        assert!(toon.contains("active=true\n"));
        assert!(toon.contains("middle=null\n"));
    }

    #[test]
    fn test_convert_nested_object() {
        let converter = Converter::new(false);

        let json = r#"{"user": {"name": "Bob", "age": 25}}"#;
        let toon = converter.convert(json).unwrap();

        assert!(toon.contains("user.name=\"Bob\"\n"));
        assert!(toon.contains("user.age=25\n"));
    }

    #[test]
    fn test_convert_array() {
        let converter = Converter::new(false);

        let json = r#"{"items": ["apple", "banana", "cherry"]}"#;
        let toon = converter.convert(json).unwrap();

        assert!(toon.contains("items.0=\"apple\"\n"));
        assert!(toon.contains("items.1=\"banana\"\n"));
        assert!(toon.contains("items.2=\"cherry\"\n"));
    }

    #[test]
    fn test_convert_empty_array() {
        let converter = Converter::new(false);

        let json = r#"{"items": []}"#;
        let toon = converter.convert(json).unwrap();

        assert!(toon.contains("items=\"[]\"\n"));
    }

    #[test]
    fn test_convert_empty_object() {
        let converter = Converter::new(false);

        let json = r#"{"data": {}}"#;
        let toon = converter.convert(json).unwrap();

        assert!(toon.contains("data=\"{}\"\n"));
    }

    #[test]
    fn test_convert_nested_array() {
        let converter = Converter::new(false);

        let json = r#"{"matrix": [[1, 2], [3, 4]]}"#;
        let toon = converter.convert(json).unwrap();

        assert!(toon.contains("matrix.0.0=1\n"));
        assert!(toon.contains("matrix.0.1=2\n"));
        assert!(toon.contains("matrix.1.0=3\n"));
        assert!(toon.contains("matrix.1.1=4\n"));
    }

    #[test]
    fn test_convert_complex_structure() {
        let converter = Converter::new(false);

        let json = r#"{
            "name": "Project",
            "version": "1.0.0",
            "authors": ["Alice", "Bob"],
            "config": {
                "debug": true,
                "timeout": 30
            }
        }"#;
        let toon = converter.convert(json).unwrap();

        assert!(toon.contains("name=\"Project\"\n"));
        assert!(toon.contains("version=\"1.0.0\"\n"));
        assert!(toon.contains("authors.0=\"Alice\"\n"));
        assert!(toon.contains("authors.1=\"Bob\"\n"));
        assert!(toon.contains("config.debug=true\n"));
        assert!(toon.contains("config.timeout=30\n"));
    }

    #[test]
    fn test_invalid_json() {
        let converter = Converter::new(false);

        let json = r#"{"invalid": json}"#;
        let result = converter.convert(json);

        assert!(result.is_err());
    }

    #[test]
    fn test_top_level_array() {
        let converter = Converter::new(false);

        let json = r#"[1, 2, 3]"#;
        let toon = converter.convert(json).unwrap();

        assert!(toon.contains("0=1\n"));
        assert!(toon.contains("1=2\n"));
        assert!(toon.contains("2=3\n"));
    }

    #[test]
    fn test_top_level_primitive() {
        let converter = Converter::new(false);

        let json = r#""hello""#;
        let toon = converter.convert(json).unwrap();

        assert_eq!(toon, "=\"hello\"\n");
    }
}

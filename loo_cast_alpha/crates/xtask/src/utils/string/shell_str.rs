use std::ops::Deref;

pub struct Canonical(String);
pub struct Formatted(String);

impl Canonical {
    /// # Purpose:
    /// Converts a (more) human-readable format into canonical content that can actually be executed.
    ///
    /// # Input assumptions:
    /// - `raw` is processed as line-oriented text.
    /// - Start and end of `raw` may have leading/trailing newline(s).
    /// - Indentation and extra per-line outer whitespace are allowed in `raw`.
    ///
    /// # Output guarantees:
    /// - Leading/trailing outer whitespace is trimmed.
    /// - Every parsed line is trimmed.
    /// - Parsed lines are rejoined with `\n`, with no preserved indentation.
    ///
    /// # Rationale:
    /// - The input can stay readable in multi-line literals, while output stays executable.
    ///
    /// # Examples:
    /// ```text
    /// input:
    /// "
    ///     echo start
    ///     cargo test
    /// "
    ///
    /// output:
    /// "echo start\ncargo test"
    /// ```
    pub fn new(raw: &str) -> Self {
        let content = raw.trim().lines().map(|line| line.trim()).collect::<Vec<_>>().join("\n");
        Self(content)
    }
}

impl Deref for Canonical {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Canonical> for Formatted {
    /// # Purpose:
    /// Converts canonical, executable content into a (more) human-readable formatted form.
    ///
    /// # Input assumptions:
    /// - Canonical content is processed as line-oriented text.
    /// - Canonical content should have no indentation; only newlines separate lines.
    /// - Start and end of canonical content may NOT have leading/trailing newline(s).
    ///
    /// # Output guarantees:
    /// - Empty canonical input produces an empty formatted output.
    /// - Non-empty output adds exactly one leading newline and one trailing newline.
    /// - Each line is indented by exactly one tab.
    ///
    /// # Rationale:
    /// - Valid formatted content can contain a leading newline.
    /// - This conversion intentionally includes those leading/trailing newlines because they are useful
    ///   when displaying multi-line shell content inside surrounding output.
    ///
    /// # Examples:
    /// ```text
    /// canonical:
    /// "echo start\ncargo test"
    ///
    /// formatted:
    /// "\n\techo start\n\tcargo test\n"
    /// ```
    fn from(canonical: Canonical) -> Self {
        if canonical.is_empty() {
            return Formatted(String::new());
        }

        let mut content = String::from("\n");

        let body = canonical.lines().map(|line| format!("\t{}", line)).collect::<Vec<_>>().join("\n");

        content.push_str(&body);
        content.push('\n');

        Formatted(content)
    }
}

impl From<&str> for Canonical {
    fn from(raw: &str) -> Self {
        Self::new(raw)
    }
}

impl Deref for Formatted {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

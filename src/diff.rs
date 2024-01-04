)(?<hunk>(:?[ @\-+\u{1b}].*
const HUNKS_REGEX: &str = r"@@ \-(?<old_start>\d+),(?<old_lines>\d+) \+(?<new_start>\d+),(?<new_lines>\d+) @@(?<header_suffix>.*
)(?<content>(:?[ \-+\u{1b}].*
        format!("{}{}{}", strip_ansi_escapes::strip_str(&self.file_header), strip_ansi_escapes::strip_str(self.header()), strip_ansi_escapes::strip_str(&self.content))
    #[test]
    fn parse_example4() {
        let diff = Diff::parse(include_str!("example4.patch"));
        assert_eq!(diff.deltas.len(), 2);
        assert_eq!(diff.deltas[0].hunks.len(), 2);
        assert_eq!(diff.deltas[1].hunks.len(), 2);
    }

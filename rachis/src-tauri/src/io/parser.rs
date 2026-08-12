use {
    crate::{errors::parser::ParserError, io::content::FileType},
    regex::Regex,
    std::{
        fmt::{Display, Formatter, Result as FmtResult},
        path::Path,
    },
};

#[derive(Debug, Default, Eq, PartialEq)]
pub enum Grammar {
    #[default]
    Markdown,
    Bbcode,
    AsciiDoc,
    Html,
    RichText,
}

#[derive(Debug, Default)]
pub struct GrammarRegion {
    grammar: Grammar,
    depth: usize,
    content_start: usize,
    content_end: usize,
    children: Vec<GrammarRegion>,
}

#[derive(Debug, Default)]
pub struct RegionTree {
    grammar: Grammar,
    regions: Vec<GrammarRegion>,
}

impl Grammar {
    fn from_id(s: impl AsRef<str>) -> Result<Grammar, ParserError> {
        match s.as_ref().to_lowercase().as_str() {
            "md" | "markdown" => Ok(Grammar::Markdown),
            "bbcode" | "bbc" => Ok(Grammar::Bbcode),
            "adoc" | "asciidoc" => Ok(Grammar::AsciiDoc),
            "html" => Ok(Grammar::Html),
            "rtf" | "richtext" => Ok(Grammar::RichText),
            _ => Err(ParserError::UnknownRegionGrammar {
                line: 0,
                content: "Error".into(),
            }),
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Grammar::Markdown => "md",
            Grammar::Bbcode => "bbcode",
            Grammar::AsciiDoc => "adoc",
            Grammar::Html => "html",
            Grammar::RichText => "rtf",
        }
    }
}

impl Display for Grammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl GrammarRegion {
    fn new(
        grammar: Grammar,
        depth: usize,
        content_start: usize,
        content_end: usize,
        children: Vec<GrammarRegion>,
    ) -> Self {
        Self {
            grammar,
            depth,
            content_start,
            content_end,
            children,
        }
    }
}

impl RegionTree {
    fn new(grammar: Grammar, regions: Vec<GrammarRegion>) -> Self {
        Self { grammar, regions }
    }

    pub fn add_region(&mut self, grammar: GrammarRegion) -> &mut Self {
        self.regions.push(grammar);
        self
    }

    pub fn parse(path: impl AsRef<Path>, tab_width: usize) -> Result<Self, ParserError> {
        let path: &Path = path.as_ref();
        let (content, ext): (&str, &str) = (
            path.file_stem().and_then(|s| s.to_str()).unwrap_or(""),
            path.extension().and_then(|e| e.to_str()).unwrap_or(""),
        );

        // Regions aren't _only_ accessible in .rachis files; all supported formats should be able to handle markers. No need to check for valid format right now, I think
        // However, if the file is a .rachis file, the first line must be a region marker, otherwise Rachis won't know what to do with it. If it's any of the other 5, then the corresponding grammar must be used
        // So I need the file, not just the contents of the file?

        // All region markers should match the RegEx r"^(?:\t|(?:[ ]{2}|[ ]{4})+)*:::(md|bbcode|adoc|html)"
        let region_tree: RegionTree = Default::default();
        let re: Regex = Regex::new(r"^((?:\t|(?:[ ]{2}|[ ]{4})+)*):::(md|bbcode|adoc|html)")?;
        match ext {
            "rachis" => {
                for (i, line) in content.lines().enumerate() {
                    if re.is_match(line) {}
                }
            }
            ext if Grammar::from_id(ext).is_ok() => {}
            _ => {}
        }

        Ok(RegionTree {
            grammar: Grammar::from_id(ext).unwrap_or_default(),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use {super::*, log::info};

    #[test]
    fn test_default_grammar() {
        let g: Grammar = Default::default();
        info!("{g:#?}");
        println!("{g:#?}");
        assert_eq!(g, Grammar::Markdown);

        let g: Result<Grammar, ParserError> = Grammar::from_id("md");
        info!("{g:#?}");
        println!("{g:?}");
        assert!(g.is_ok());
        assert!(g.is_ok_and(|g| g == Grammar::Markdown));

        let g: Result<Grammar, ParserError> = Grammar::from_id("darkmown");
        info!("{g:#?}");
        println!("{g:?}");
        assert!(g.is_err());
    }

    #[test]
    fn test_grammar_region() {
        let gr: GrammarRegion = Default::default();
        info!("{gr:?}");
        println!("{gr:#?}");
        assert_eq!(gr.grammar, Grammar::Markdown);
        assert_eq!(gr.depth, 0);
        assert_eq!(gr.content_start, 0);
        assert_eq!(gr.content_end, 0);
        assert!(gr.children.is_empty());
    }
}

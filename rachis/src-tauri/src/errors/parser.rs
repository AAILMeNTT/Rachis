use {
    regex::Error as RegexError,
    std::{
        error::Error,
        fmt::{Display, Formatter, Result as FmtResult},
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParserError {
    /// Don't use tabs and spaces in the same document its just gonna piss everyone off
    MixedIndentation { line: usize },
    /// `:::` with an unrecognised grammar after it
    UnknownRegionGrammar { line: usize, content: String },
    /// `:::` with nothing after it
    EmptyRegionMarker { line: usize },
    /// `:::` with a buncha bullshit after a valid grammar
    TrailingRegionMarkerContent { line: usize, content: String },
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ParserError::MixedIndentation { line } => {
                write!(f, "line {line}: mixed tabs and spaces in indent prefix")
            }
            ParserError::UnknownRegionGrammar { line, content } => {
                write!(f, "line {line}: unknown grammar id `{content}`")
            }
            ParserError::EmptyRegionMarker { line } => {
                write!(f, "line {line}: `:::` marker with no grammar")
            }
            ParserError::TrailingRegionMarkerContent { line, content } => write!(
                f,
                "line {line}: unexpected content after grammar: `{content}`"
            ),
        }
    }
}

impl Error for ParserError {}

impl From<RegexError> for ParserError {
    fn from(error: RegexError) -> Self {
        ParserError::UnknownRegionGrammar { line: 0, content: error.to_string() }
    }
}

use {
    serde::{Deserialize, Serialize},
    std::fmt,
};

/// A parsed tag from [Rachis] text.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// The beginning character of the tag.
    pub prefix: char,
    /// The raw content of the entire tag.
    pub raw: String,
    /// The content between the delimiters.
    pub content: String,
    /// The directory to the Rachis.
    pub folder_path: Vec<String>,
    /// The display text for the Rachis.
    pub display_text: String,
    /// The index of the Rachis.
    pub index: String,
    /// The lock status of the Rachis.
    /// 0 - unrestricted (any Rachis can have this same name)
    /// 1 - local restriction (any Rachis of the same type as this must have a different name)
    /// 2 - global restriction (no other Rachises can have this name)
    pub lock: u8,
}

/// Methods for the Tag struct.
///
/// # Functions
///
/// - [Tag::validate_prefix](Tag::validate_prefix) - Validates the prefix of a tag.
/// - [Tag::parse](Tag::parse) - Parses a tag.
impl Tag {
    /// Validates the prefix of a tag.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Returns an error if the prefix is invalid, otherwise returns Ok(()).
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Tag::validate_prefix()
    pub fn validate_prefix(&self) -> Result<(), String> {
        match self.prefix {
            'c' | 'e' | 'l' | 'i' | 'n' => Ok(()),
            _ => Err(String::from(
                "Prefix must be one of 'c', 'e', 'l', 'i', or 'n'!",
            )),
        }
    }

    /// Parses a tag.
    ///
    /// # Arguments
    ///
    /// - s: [`impl AsRef<str>`](str) - The tag to parse.
    ///
    /// # Returns
    ///
    /// - `Option<Tag>` - Returns the parsed tag if successful, otherwise returns None.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Tag::parse()
    pub fn parse(s: impl AsRef<str>) -> Option<Self> {
        let s: &str = s.as_ref();

        // If the whole tag is shorter than 4 characters, return None
        // All tags are comprised of at least 4 elements: the prefix (1+ char(s)), the delimiters (2
        // chars), and the content (1+ char(s))
        if s.len() < 4 {
            return None;
        }

        // let modifiers: Vec<char> = vec!['!', '~', '|', '.', ':', '/'];
        const MODIFIERS: [char; 6] = ['!', '~', '|', '.', ':', '/'];

        // Split string into chars
        let mut chars = s.chars();
        // Assign prefix to the first char
        let prefix: char = chars.next()?;

        if chars.next()? != '!' {
            return None;
        };

        // The content between the delimiters
        let mut content: String = String::new();
        // Whether the current character is escaped
        let mut escaped: bool = false;
        // Whether the next character(s) is/are part of an index
        let mut is_index: bool = false;
        // Whether the next character(s) is/are part of the display text
        let mut is_display_text: bool = false;
        // A vector representing the folder path of the Rachis
        let mut folder_path: Vec<String> = Vec::new();
        // The index of the Rachis
        let mut index: String = String::new();
        // The display text of the Rachis
        let mut display_text: String = String::new();
        // The lock status of the Rachis
        let mut lock: u8 = 0;

        // Whether the character should be pushed to the content string
        let mut push_to_content: bool;

        // For each character in the string...
        for c in chars {
            // Default to not push character to content string
            push_to_content = false;

            // If the previous character is a '\', push the current character as it is
            // This has to be first so that it immediately bypasses any other modifiers
            if escaped {
                if is_display_text {
                    display_text.push(c);
                } else if is_index && c.is_ascii_digit() {
                    index.push(c);
                } else {
                    push_to_content = true;
                }
                escaped = false;
            } else {
                // Otherwise, the character isn't escaped
                match c {
                    // If this character is a '\', mark the next character as escaped
                    '\\' => {
                        escaped = true;
                    }
                    // If this character is a '~', mark the next character(s) as part of the index
                    '~' => {
                        println!(
                            "Modifier {c} found: following integer(s) are part of this Rachis' index."
                        );
                        // Mark the next characters as part of the index
                        is_index = true;
                    }
                    '|' => {
                        println!(
                            "Modifier {c} found: following text is part of this Rachis' display text."
                        );
                        // Mark the following characters as part of the display text
                        is_display_text = true;
                    }
                    // If this character is a '.', mark the next character as a lock (type 1: locally unique)
                    '.' => {
                        println!("Modifier {c} found: this Rachis is locally unique.");
                        lock = 1;
                    }
                    // If this character is a ':', mark the next character as a lock (type 2: globally unique)
                    ':' => {
                        println!("Modifier {c} found: this Rachis is globally unique.");
                        lock = 2;
                    }
                    // If this character is a '/', transfer the current content to the folder path vector
                    '/' => {
                        println!("Modifier {c} found: this Rachis is in a folder.");
                        folder_path.push(content.clone());
                        content.clear();
                    }
                    // If this character is a '!', end the tag
                    '!' => {
                        println!("Modifier {c} found: end of tag reached.");

                        if content.is_empty() {
                            return None;
                        }

                        return Some(Tag {
                            prefix,
                            content,
                            raw: s.to_string(),
                            folder_path,
                            index,
                            display_text,
                            lock,
                        });
                    }
                    // Otherwise, simply push this character to the String
                    _ => {
                        // If the character is part of the index and is a digit, append to index string
                        if is_index && c.is_ascii_digit() {
                            index.push(c);
                        } else {
                            is_index = false;
                        }

                        // If the character is part of the index and not a modifier, add to the
                        // display text string
                        if is_display_text && !MODIFIERS.contains(&c) {
                            display_text.push(c);
                        } else {
                            is_display_text = false;
                        }

                        if !is_index && !is_display_text {
                            push_to_content = true;
                        }
                    }
                }
            }
            if push_to_content {
                // Push the current character to the content string
                println!("Pushing \"{c}\" to content.");
                content.push(c);
            }
        }

        // At this point, no closing '!' was found
        None
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Prefix: {}\nContent: {}\nRaw: {}\nFolder Path: {}\nIndex: {}\nDisplay Text: {}\nLock status: {}",
            self.prefix,
            self.content,
            self.raw,
            self.folder_path.join("/-/"),
            self.index,
            self.display_text,
            self.lock
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests valid range delimiter usage of a [Tag].
    #[test]
    fn valid_range_delimiters() {
        // ———————— Valid Tag ————————
        let valid_tag: Option<Tag> = Tag::parse("c!Twilight Sparkle!");
        // Assert that the second and last characters are delimiters
        assert!(valid_tag.is_some());
    }

    /// Tests invalid range delimiter usage of a [Tag].
    #[test]
    fn invalid_range_delimiters() {
        // ———————— Invalid Tag 1 ————————
        let invalid_tag: Option<Tag> = Tag::parse("c-Twilight Sparkle!");
        // Assert that the second char is not a delimiter
        assert!(invalid_tag.is_none());

        // ———————— Invalid Tag 2 ————————
        let invalid_tag: Option<Tag> = Tag::parse("c!Twilight Sparkle-");
        // Assert that the last char is not a delimiter
        assert!(invalid_tag.is_none());
    }

    /// Tests the valid prefixes of a [Tag].
    #[test]
    fn valid_prefixes() {
        // ———————— Valid Tag ————————
        let char_tag: Option<Tag> = Tag::parse("c!Twilight Sparkle!");
        let event_tag: Option<Tag> = Tag::parse("e!Twilight Sparkle!");
        let loc_tag: Option<Tag> = Tag::parse("l!Twilight Sparkle!");
        let item_tag: Option<Tag> = Tag::parse("i!Twilight Sparkle!");
        let note_tag: Option<Tag> = Tag::parse("n!Twilight Sparkle!");
        // Assert that the first character is one of 'c', 'e', 'l', 'i', or 'n'
        assert!(char_tag.unwrap().validate_prefix().is_ok());
        assert!(event_tag.unwrap().validate_prefix().is_ok());
        assert!(loc_tag.unwrap().validate_prefix().is_ok());
        assert!(item_tag.unwrap().validate_prefix().is_ok());
        assert!(note_tag.unwrap().validate_prefix().is_ok());
    }

    /// Tests the invalid prefixes of a [Tag].
    #[test]
    fn invalid_prefixes() {
        // ———————— Invalid Tag ————————
        let tag: Option<Tag> = Tag::parse("x!Twilight Sparkle!");
        // Assert that the first character is not one of 'c', 'e', 'l', 'i', or 'n'
        assert!(tag.unwrap().validate_prefix().is_err());
    }

    /// Tests a [Tag] without an index.
    #[test]
    fn no_index() {
        // ———————— No Index ————————
        let tag: Option<Tag> = Tag::parse("c!Twilight Sparkle!");
        // Assert that the index is an empty String
        assert_eq!(tag.unwrap().index.is_empty(), true);
    }

    /// Tests a [Tag] with an index.
    #[test]
    fn with_index() {
        // ———————— With Index ————————
        let tag2: Option<Tag> = Tag::parse("c!Twilight Sparkle~2!");
        let tag203452: Option<Tag> = Tag::parse("c!Twilight Sparkle~203452!");
        // Assert that the index is not an empty String
        assert_eq!(tag2.unwrap().index.is_empty(), false);
        assert_eq!(tag203452.unwrap().index.is_empty(), false);
    }

    /// Tests a [Tag] with content.
    #[test]
    fn tag_with_content() {
        // ———————— With Content ————————
        let tag_with_content: Option<Tag> = Tag::parse("e!Crystal Kingdom's Return\\!~2!");
        println!("{tag_with_content:#?}");
        // Assert that the content is not an empty String
        assert_eq!(
            tag_with_content.unwrap().content,
            "Crystal Kingdom's Return!"
        );
    }

    /// Tests a [Tag] without content.
    #[test]
    fn tag_with_no_content() {
        // ———————— No Content ————————
        let tag_no_content1: Option<Tag> = Tag::parse("c!~2!");
        let tag_no_content2: Option<Tag> = Tag::parse("c!.!");
        let tag_no_content3: Option<Tag> = Tag::parse("c!:!");
        let tag_no_content4: Option<Tag> = Tag::parse("c!/!");
        let tag_no_content5: Option<Tag> = Tag::parse("c!/~10:!");
        println!("{:?}", tag_no_content1);
        println!("{:?}", tag_no_content2);
        println!("{:?}", tag_no_content3);
        println!("{:?}", tag_no_content4);
        println!("{:?}", tag_no_content5);
        // Assert that tags are None
        assert_eq!(tag_no_content1.is_none(), true);
        assert_eq!(tag_no_content2.is_none(), true);
        assert_eq!(tag_no_content3.is_none(), true);
        assert_eq!(tag_no_content4.is_none(), true);
        assert_eq!(tag_no_content5.is_none(), true);
    }

    /// Tests a [Tag] with display text.
    #[test]
    fn display_text() {
        // ———————— Display Text ————————
        let tag_with_display: Option<Tag> = Tag::parse("c!Twilight Sparkle|Equestrian Princess!");
        println!("{tag_with_display:#?}");
        // Assert that there is display text
        assert_eq!(tag_with_display.unwrap().display_text.is_empty(), false);
    }

    /// Tests a [Tag] without display text.
    #[test]
    fn no_display_text() {
        // ———————— No Display Text ————————
        let tag_without_display: Option<Tag> =
            Tag::parse("c!Twilight Sparkle, Equestrian Princess!");
        println!("{tag_without_display:#?}");
        // Assert that there is no display text
        assert_eq!(tag_without_display.unwrap().display_text.is_empty(), true);
    }

    /// Tests a [Tag] without a folder path.
    #[test]
    fn folder_path_no_subdirectory() {
        // ——————— No Subdirectory ————————
        let tag_without_folder: Option<Tag> = Tag::parse("c!Twilight Sparkle!");
        println!("{tag_without_folder:#?}");
        // Assert that there is no folder path
        assert_eq!(tag_without_folder.unwrap().folder_path.is_empty(), true);
    }

    /// Tests a [Tag] with a folder path.
    #[test]
    fn folder_path_single_subdirectory() {
        // ——————— Single Subdirectory ————————
        let tag_with_folder: Option<Tag> = Tag::parse("c!Mane 6/Twilight Sparkle!");
        println!("{tag_with_folder:#?}");
        // Assert that there is a folder path
        assert_eq!(tag_with_folder.unwrap().folder_path, vec!["Mane 6"]);
    }

    /// Tests a [Tag] with a multiple level folder path.
    #[test]
    fn folder_path_multiple_subdirectories() {
        // ——————— Multiple Subdirectories ————————
        let tag_with_folders: Option<Tag> = Tag::parse("c!Protagonists/Mane 6/Twilight Sparkle!");
        println!("{tag_with_folders:#?}");
        // Assert that there is a folder path
        assert_eq!(
            tag_with_folders.unwrap().folder_path,
            vec!["Protagonists", "Mane 6"]
        );
    }

    /// Tests a [Tag] with an escaped folder path.
    #[test]
    fn folder_path_escaped_subdirectory() {
        // ——————— Escaped Subdirectory ————————
        let tag_with_escaped_folder: Option<Tag> = Tag::parse(&String::from(
            "c!Protagonists\\/Main Characters/Mane 6/Twilight Sparkle!",
        ));
        println!("{tag_with_escaped_folder:#?}");
        // Assert that there is a folder path
        assert_eq!(
            tag_with_escaped_folder.unwrap().folder_path,
            vec!["Protagonists/Main Characters", "Mane 6"]
        );
    }

    /// The hell parse
    #[test]
    fn the_hell_parse() {
        // Oh hell
        let tag: Option<Tag> = Tag::parse(&String::from(
            "l!Equestria/Ponyville\\/\"The Apple Core\"/Golden Oaks Library\\!~3|Twilight's House (v\\.2)\\!:!",
        ));
        println!("{}", format!("{}", tag.as_ref().unwrap().to_string()));
        // Verify that the tag is parsed correctly:
        // - The tag has a location (l)
        // - The tag has a folder path (/)
        // - The tag has an index specified (~3)
        // - The tag has display text (|)
        // - The tag has various escapes (\)
        // - The tag name is globally restricted (:)
        assert_eq!(tag.clone().unwrap().prefix, 'l');
        assert_eq!(tag.clone().unwrap().content, "Golden Oaks Library!");
        assert_eq!(
            tag.clone().unwrap().raw,
            "l!Equestria/Ponyville\\/\"The Apple Core\"/Golden Oaks Library\\!~3|Twilight's House (v\\.2)\\!:!"
        );
        assert_eq!(
            tag.clone().unwrap().folder_path,
            vec!["Equestria", "Ponyville/\"The Apple Core\""]
        );
        assert_eq!(tag.clone().unwrap().index, "3");
        assert_eq!(tag.clone().unwrap().display_text, "Twilight's House (v.2)!");
        assert_eq!(tag.clone().unwrap().lock, 2);
    }
}

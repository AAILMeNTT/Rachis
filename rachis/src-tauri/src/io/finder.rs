use {
    regex::Regex,
    std::path::{Path, PathBuf},
    walkdir::{DirEntry, Error as WalkError, WalkDir},
};

#[derive(Clone, Debug, Default)]
pub struct Finder {
    path: PathBuf,
    skip_hidden_files: Option<bool>,
    skip_hidden_dirs: Option<bool>,
    skip_files: Option<bool>,
    skip_dirs: Option<bool>,
    exclude_pattern: Option<Regex>,
    extensions: Option<Vec<String>>,
    max_depth: Option<usize>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FinderResult {
    pub files: Vec<PathBuf>,
    pub directories: Vec<PathBuf>,
}

impl Finder {
    /// Creates a new `Finder` that will search for files in the given path.
    ///
    /// # Arguments
    ///
    /// - `path`: [`impl AsRef<Path>`](Path) - The path to search for files in.
    ///
    /// # Returns
    ///
    /// A new `Finder` instance.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            ..Default::default()
        }
    }

    /// Skips hidden files and directories.
    pub fn skip_hidden(&mut self) -> &mut Self {
        self.skip_hidden_files = Some(true);
        self.skip_hidden_dirs = Some(true);
        self
    }

    /// Skips hidden files.
    #[allow(dead_code)]
    pub fn skip_hidden_files(&mut self) -> &mut Self {
        self.skip_hidden_files = Some(true);
        self
    }

    /// Skips hidden directories.
    pub fn skip_hidden_dirs(&mut self) -> &mut Self {
        self.skip_hidden_dirs = Some(true);
        self
    }

    /// Skips files.
    #[allow(dead_code)]
    pub fn skip_files(&mut self) -> &mut Self {
        self.skip_files = Some(true);
        self
    }

    /// Skips directories.
    #[allow(dead_code)]
    pub fn skip_dirs(&mut self) -> &mut Self {
        self.skip_dirs = Some(true);
        self
    }

    /// Excludes files matching the given pattern.
    #[allow(dead_code)]
    pub fn exclude_pattern(&mut self, pattern: impl AsRef<str>) -> &mut Self {
        self.exclude_pattern =
            Some(Regex::new(pattern.as_ref()).expect("Invalid regex pattern for exclude_pattern!"));
        self
    }

    /// Includes files with the given extensions.
    #[allow(dead_code)]
    pub fn extensions(
        &mut self,
        extensions: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> &mut Self {
        self.extensions = Some(
            extensions
                .into_iter()
                .map(|e| e.as_ref().to_string())
                .collect(),
        );
        self
    }

    /// Limits the maximum traversal depth from the root.
    ///
    /// # Arguments
    ///
    /// - `depth`: `usize` — Maximum directory depth to traverse.
    ///   `0` means only the root directory itself.
    ///   `1` means the root and its immediate children.
    ///   Default (unset) is unlimited.
    #[allow(dead_code)]
    pub fn depth(&mut self, depth: usize) -> &mut Self {
        self.max_depth = Some(depth);
        self
    }

    /// Walks the directory tree and collects matching paths.
    pub fn find(&self) -> Result<FinderResult, WalkError> {
        let mut result: FinderResult = FinderResult::new();

        let mut walker = WalkDir::new(&self.path);
        if let Some(depth) = self.max_depth {
            walker = walker.max_depth(depth);
        }

        for entry in walker
            .into_iter()
            .filter_entry(|e| self.is_allowed(e))
            .filter_map(Result::ok)
            .filter(|e| self.include_in_result(e))
        {
            match entry.file_type().is_dir() {
                true => result.directories.push(entry.into_path()),
                false => result.files.push(entry.into_path()),
            }
        }

        Ok(result)
    }

    /// Walks the directory tree and returns the first matching file path, or `None`.
    #[allow(dead_code)]
    pub fn find_first(&self) -> Result<Option<PathBuf>, WalkError> {
        let mut walker = WalkDir::new(&self.path);
        if let Some(depth) = self.max_depth {
            walker = walker.max_depth(depth);
        }

        Ok(walker
            .into_iter()
            .filter_entry(|e| self.is_allowed(e))
            .filter_map(Result::ok)
            .find(|e| e.file_type().is_file() && self.include_in_result(e))
            .map(|e| e.into_path()))
    }

    #[allow(dead_code)]
    pub fn count(&self) -> Result<usize, WalkError> {
        let mut walker = WalkDir::new(&self.path);
        if let Some(depth) = self.max_depth {
            walker = walker.max_depth(depth);
        }

        Ok(walker
            .into_iter()
            .filter_entry(|e| self.is_allowed(e))
            .filter_map(Result::ok)
            .filter(|e| self.include_in_result(e))
            .count())
    }

    /// Determines whether a single entry (file or directory) passes the hidden-entry filter.
    ///
    /// - For directories, checks `skip_hidden_dirs`, skipping over hidden directories if Some(true)
    /// - For files, checks `skip_hidden_files`, skipping over hidden files if Some(true)
    ///
    /// # Arguments
    ///
    /// - `entry`: [`&DirEntry`](DirEntry) - The directory entry to check
    fn is_allowed(&self, entry: &DirEntry) -> bool {
        let name: &str = entry.file_name().to_str().unwrap_or("");
        let is_hidden: bool = name.starts_with('.');

        if entry.file_type().is_dir() {
            // Skip hidden directories if that filter is on
            return !(self.skip_hidden_dirs.unwrap_or_default() && is_hidden);
        }

        // Everything below here applies only to files

        if self.skip_files.unwrap_or(false) {
            return false;
        }
        if self.skip_hidden_files.unwrap_or(false) && is_hidden {
            return false;
        }

        // Exclude files matching the pattern
        if let Some(re) = &self.exclude_pattern {
            if re.is_match(entry.path().to_str().unwrap_or("")) {
                return false;
            }
        }

        // If the user included any extensions, ensure the entry's file extension matches
        if let Some(exts) = &self.extensions {
            let ext: Option<&str> = entry.path().extension().and_then(|e| e.to_str());
            if !exts.iter().any(|e| Some(e.as_str()) == ext) {
                return false;
            }
        }

        true
    }

    /// Determines whether an entry should appear in the final result set.
    ///
    /// This is a post-walk filter that only affects what gets collected into
    /// [`FinderResult`]. Primarily for `skip_dirs`, to allow directories to be
    /// traversed to capture their files, yet excluded from the output.
    ///
    /// # Arguments
    ///
    /// - `entry`: [`&DirEntry`](DirEntry) - The directory entry to check.
    fn include_in_result(&self, entry: &DirEntry) -> bool {
        !(entry.file_type().is_dir() && self.skip_dirs.unwrap_or(false))
    }
}

impl FinderResult {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn add_file(&mut self, path: impl AsRef<Path>) {
        self.files.push(path.as_ref().to_path_buf());
    }

    #[allow(dead_code)]
    fn add_dir(&mut self, path: impl AsRef<Path>) {
        self.directories.push(path.as_ref().to_path_buf());
    }

    #[allow(dead_code)]
    fn count(&self) -> usize {
        self.files.len() + self.directories.len()
    }
}

#[cfg(test)]
mod proptests {
    use {
        super::*,
        prop::{bool, collection, sample},
        proptest::prelude::*,
        std::{env, fs},
        uuid::Uuid,
    };

    struct TempDir(PathBuf);

    impl TempDir {
        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    /// Describes a single file to create in the temp directory.
    #[derive(Debug)]
    struct FileSpec {
        name: String,
        hidden: bool,
        ext: Option<String>,
    }

    impl FileSpec {
        /// Builds the actual filename from name, hidden, and extension.
        fn filename(&self) -> String {
            let mut base: String = match self.hidden {
                true => format!(".{}", self.name),
                false => self.name.clone(),
            };
            if let Some(ext) = &self.ext {
                base = format!("{base}.{ext}");
            }
            base
        }
    }

    /// Strategy: 80% of entries get an extension, 20% don't.
    /// 30% of entries are hidden across both groups.
    fn file_spec() -> impl Strategy<Value = FileSpec> {
        prop_oneof![
            4 => (
                "[a-z]{1,10}",
                sample::select(&["rachis", "md", "json", "txt"]),
                bool::weighted(0.3)
            ).prop_map(|(name, ext, hidden)| FileSpec {
                name,
                hidden,
                ext: Some(ext.to_string()),
            }),
            1 => ("[a-z]{1,10}", bool::weighted(0.3)).prop_map(|(name, hidden)| FileSpec {
                name,
                hidden,
                ext: None,
            }),
        ]
    }

    /// Creates all files described by `specs` in a new temp directory.
    fn create_files(specs: &[FileSpec]) -> TempDir {
        let dir: PathBuf = env::temp_dir().join(format!("proptest_finder_{}", Uuid::new_v4()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        for spec in specs {
            fs::write(dir.join(spec.filename()), "").unwrap();
        }

        TempDir(dir)
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]

        #[test]
        fn skip_hidden_files_excludes_dotfiles(
            specs in collection::vec(file_spec(), 0..20),
        ) {
            let tree = create_files(&specs);
            let result = Finder::new(tree.path()).skip_hidden_files().find().unwrap();

            for file in &result.files {
                let name = file.file_name().and_then(|s| s.to_str()).unwrap_or("");
                prop_assert!(!name.starts_with('.'), "expected no hidden files, found {name:?}");
            }
        }

        #[test]
        fn skip_dirs_excludes_root_directory(
            // count in 1..20usize,
            specs in collection::vec(file_spec(), 0..20),
        ) {
            let tree = create_files(&specs);
            let result = Finder::new(tree.path()).skip_dirs().find().unwrap();

            prop_assert!(
                result.directories.is_empty(),
                "skip_dirs should exclude all directories"
            );
        }

        #[test]
        fn skip_dirs_with_extension_filter(
            specs in collection::vec(file_spec(), 0..20),
        ) {
            let tree = create_files(&specs);
            let result = Finder::new(tree.path()).skip_dirs().extensions(["rachis"]).find().unwrap();

            prop_assert!(result.directories.is_empty(), "no dirs in results");

            for file in &result.files {
                prop_assert_eq!(
                    file.extension().and_then(|s| s.to_str()),
                    Some("rachis"),
                );
            }
        }

        #[test]
        fn extension_filter_only_returns_matching(
            specs in collection::vec(file_spec(), 0..20),
        ) {
            let tree = create_files(&specs);
            let result = Finder::new(tree.path()).skip_hidden().extensions(["rachis"]).find().unwrap();

            // Every returned file must be .rachis
            for file in &result.files {
                prop_assert_eq!(
                    file.extension().and_then(|s| s.to_str()),
                    Some("rachis"),
                    "non-.rachis file in filtered results",
                );
            }

            // Every non-hidden .rachis entry must appear
            let rachis_count = specs.iter()
                .filter(|s| s.ext.as_deref() == Some("rachis") && !s.hidden)
                .count();
            prop_assert_eq!(
                result.files.len(),
                rachis_count,
                "expected {} non-hidden .rachis files, found {}",
                rachis_count,
                result.files.len(),
            );
        }
    }
}

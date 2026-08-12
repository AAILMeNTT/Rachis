pub mod ops;

use {
    crate::errors::tree::TreeError,
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter, Result as FmtResult},
    ts_rs::TS,
    uuid::Uuid,
};

/// The root of a Workspace tree. Always has exactly one root node.
///
/// # Fields
///
/// - `root`: [`TreeNode`] - The root of the Workspace tree.
#[derive(Clone, Debug, Default, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Tree {
    /// The root of the Workspace tree.
    root: TreeNode,
}

impl Tree {
    /// Finds a node by its ID, returning a reference if found.
    ///
    /// # Returns
    ///
    /// - [`Ok(&TreeNode)`](TreeNode) - A reference to the node if found
    /// - [`Err(TreeError::NodeNotFound)`](TreeError::NodeNotFound) - An error if the node is not found
    pub fn find_node(&self, node_id: impl AsRef<Uuid>) -> Result<&TreeNode, TreeError> {
        let node_id: &Uuid = node_id.as_ref();
        self.root
            .get_nodes()
            .into_iter()
            .find(|n| n.get_id() == *node_id)
            .ok_or(TreeError::NodeNotFound(*node_id))
    }

    /// Finds a node by its ID, returning a mutable reference if found.
    ///
    /// # Fields
    ///
    /// - `node`: [`&mut TreeNode`](TreeNode) - The node to search within
    /// - `target_id`: [`Uuid`](Uuid) - The ID of the node to find
    ///
    /// # Returns
    ///
    /// - [`Ok(&mut TreeNode)`](TreeNode) - A mutable reference to the node if found
    /// - [`Err(TreeError::NodeNotFound)`](TreeError::NodeNotFound) - An error if the node is not found
    pub fn find_node_mut(
        node: &mut TreeNode,
        target_id: impl AsRef<Uuid>,
    ) -> Result<&mut TreeNode, TreeError> {
        let target_id: &Uuid = target_id.as_ref();
        // Determine the type of TreeNode the node is
        match node {
            // If the current node is a Branch matching the target_id, return that node
            TreeNode::Branch(b) if b.id == *target_id => Ok(node),
            // If the current node is a Branch but not the target, recursively search its children
            TreeNode::Branch(b) => {
                // For every child in the node...
                for child in &mut b.children {
                    // If its ID matches the target_id, return that child
                    if let Ok(found) = Self::find_node_mut(child, target_id) {
                        return Ok(found);
                    }
                }
                // If there are no matching children, throw an error
                Err(TreeError::NodeNotFound(*target_id))
            }
            // If the current node is a Leaf matching the target_id, return that node
            TreeNode::Leaf(l) if l.id == *target_id => Ok(node),
            // If the current node is a leaf that doesn't match the target_id, return None
            TreeNode::Leaf(_) => Err(TreeError::NodeNotFound(*target_id)),
        }
    }

    pub fn find_branch(&self, branch_id: impl AsRef<Uuid>) -> Result<&Branch, TreeError> {
        let branch_id: &Uuid = branch_id.as_ref();
        self.root
            .get_branches()
            .into_iter()
            .find(|b| &b.id == branch_id)
            .ok_or(TreeError::BranchNotFound(*branch_id))
    }

    pub fn find_leaf(&self, leaf_id: impl AsRef<Uuid>) -> Result<&Leaf, TreeError> {
        let leaf_id: &Uuid = leaf_id.as_ref();
        self.root
            .get_leaves()
            .into_iter()
            .find(|l| &l.id == leaf_id)
            .ok_or(TreeError::LeafNotFound(*leaf_id))
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Tree root: {}", self.root)
    }
}

/// A node in the Workspace tree, either a [`Branch`] or a [`Leaf`].
///
/// - [`Branch`]: Splits its space among child nodes along a direction.
/// - [`Leaf`]: A widget instance at a position in the tree.
#[derive(Clone, Debug, Serialize, Deserialize, TS, PartialEq, Eq)]
#[ts(export)]
pub enum TreeNode {
    /// A container that splits space among its children
    Branch(Branch),
    /// A widget instance at a position in the tree
    Leaf(Leaf),
}

impl TreeNode {
    pub fn is_branch(&self) -> bool {
        matches!(self, TreeNode::Branch(_))
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self, TreeNode::Leaf(_))
    }

    pub fn as_leaf(&self) -> Result<&Leaf, TreeError> {
        match self {
            TreeNode::Leaf(leaf) => Ok(leaf),
            _ => Err(TreeError::WrongNodeKind {
                expected: "Leaf".into(),
                actual: self.to_string(),
            }),
        }
    }

    pub fn as_branch(&self) -> Result<&Branch, TreeError> {
        match self {
            TreeNode::Branch(branch) => Ok(branch),
            _ => Err(TreeError::WrongNodeKind {
                expected: "Branch".into(),
                actual: self.to_string(),
            }),
        }
    }

    pub fn as_leaf_mut(&mut self) -> Result<&mut Leaf, TreeError> {
        match self {
            TreeNode::Leaf(leaf) => Ok(leaf),
            _ => Err(TreeError::WrongNodeKind {
                expected: "Leaf".into(),
                actual: self.to_string(),
            }),
        }
    }

    pub fn as_branch_mut(&mut self) -> Result<&mut Branch, TreeError> {
        match self {
            TreeNode::Branch(branch) => Ok(branch),
            _ => Err(TreeError::WrongNodeKind {
                expected: "Branch".into(),
                actual: self.to_string(),
            }),
        }
    }

    pub fn get_nodes(&self) -> Vec<&TreeNode> {
        let mut nodes = vec![self];
        match self {
            TreeNode::Branch(branch) => nodes.extend(branch.get_nodes()),
            TreeNode::Leaf(_) => {}
        }
        nodes
    }

    pub fn get_nodes_mut(&mut self) -> &mut [TreeNode] {
        match self {
            TreeNode::Branch(branch) => &mut branch.children,
            TreeNode::Leaf(_) => &mut [],
        }
    }

    /// Return all leaves stemming from this node downwards.
    ///
    /// If this node is a leaf, returns a single-element vector containing itself.
    /// If this node is a branch, returns all leaves from all children.
    pub fn get_leaves(&self) -> Vec<&Leaf> {
        match self {
            TreeNode::Branch(branch) => branch.get_leaves(),
            TreeNode::Leaf(leaf) => vec![leaf],
        }
    }

    /// Return all branches stemming from this node downwards.
    ///
    /// If this node is a leaf, returns an empty vector.
    /// If this node is a branch, returns itself along with all branches from all its children.
    pub fn get_branches(&self) -> Vec<&Branch> {
        match self {
            // If this node is a branch...
            TreeNode::Branch(branch) => {
                // Initialise a vector with this branch
                let mut branches = vec![branch];
                // For every child, extend the branches vector with its own branches
                for child in &branch.children {
                    branches.extend(child.get_branches());
                }
                branches
            }
            TreeNode::Leaf(_) => Vec::new(),
        }
    }

    fn get_id(&self) -> Uuid {
        match self {
            TreeNode::Branch(branch) => branch.id,
            TreeNode::Leaf(leaf) => leaf.id,
        }
    }
}

impl Default for TreeNode {
    fn default() -> Self {
        Self::Leaf(Default::default())
    }
}

impl Display for TreeNode {
    /// Formats TreeNode as a concise string for log messages, error reporting, etc.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TreeNode::Branch(b) => write!(f, "{b}"),
            TreeNode::Leaf(l) => write!(f, "{l}"),
        }
    }
}

/// A container node that splits its space among children.
///
/// Fills the assigned area along its [`Direction`], drawing resize handles
/// between children. A Branch with one child renders that child at full size
/// with no split chrome.
///
/// # Fields
///
/// - `id`: [`Uuid`] - Unique identifier for this Branch
/// - `direction`: [`Direction`] - The direction along which children are laid out (horizontal or vertical)
/// - `children`: Vec<[`TreeNode`]> - Child nodes in order. 0..N children
/// - `ratios`: Vec<f32> - Relative sizes for each child. Must match `children.len()` when active
#[derive(Clone, Debug, Serialize, Deserialize, TS, PartialEq, Eq)]
#[ts(export)]
pub struct Branch {
    /// Unique identifier for this Branch
    pub id: Uuid,
    /// The direction along which children are laid out (horizontal or vertical)
    pub direction: Direction,
    /// Child nodes in order. 0..N children
    pub children: Vec<TreeNode>,
    /// Relative sizes for each child. Must match `children.len()` when active
    pub ratios: Vec<u32>,
}

impl Branch {
    /// Returns this Branch as a [`TreeNode::Branch`](TreeNode).
    pub fn as_node(&self) -> TreeNode {
        TreeNode::Branch(self.clone())
    }

    /// Returns a reference to each child node in this Branch.
    pub fn get_nodes(&self) -> Vec<&TreeNode> {
        self.children.iter().collect::<Vec<&TreeNode>>()
    }

    /// Returns a reference to each child branch in this Branch.
    pub fn get_branches(&self) -> Vec<&Branch> {
        let mut branches: Vec<&Branch> = Vec::new();
        for child in &self.children {
            if let TreeNode::Branch(b) = child {
                branches.push(b);
                branches.extend(b.get_branches());
            }
        }
        branches
    }

    /// Returns a reference to each leaf node in this Branch.
    pub fn get_leaves(&self) -> Vec<&Leaf> {
        self.children
            .iter()
            .flat_map(|node| node.get_leaves())
            .collect::<Vec<&Leaf>>()
    }
}

impl Default for Branch {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            direction: Direction::default(),
            children: Vec::new(),
            ratios: Vec::new(),
        }
    }
}

impl Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}")
    }
}

/// A leaf node representing a Widget instance at a position in the Workspace tree.
///
/// # Fields
///
/// - `id`: [`Uuid`] - Unique identifier for this Leaf
/// - `widget_type`: [`WidgetType`] - The type of widget this leaf holds
/// - `widget_instance_id`: Option<[`Uuid`]> - Which specific document or context this widget displays
///
///     - Editor widgets: the UUID of the Rachis being edited
///     - Notes widgets: the UUID of the note being viewed
///     - Picker/Empty widgets: `None`
#[derive(Clone, Debug, Serialize, Deserialize, TS, PartialEq, Eq)]
#[ts(export)]
pub struct Leaf {
    /// Unique identifier for this Leaf
    pub id: Uuid,
    /// The type of widget this leaf holds
    pub widget_type: WidgetType,
    /// Which specific document or context this widget displays
    ///
    /// - Editor widgets: the UUID of the Rachis being edited
    /// - Notes widgets: the UUID of the note being viewed
    /// - Picker/Empty widgets: `None`
    pub widget_instance_id: Option<Uuid>,
}

impl Leaf {
    /// Returns this Leaf as a [`TreeNode::Leaf`](TreeNode).
    pub fn as_node(&self) -> TreeNode {
        TreeNode::Leaf(self.clone())
    }
}

impl Default for Leaf {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            widget_type: WidgetType::default(),
            widget_instance_id: None,
        }
    }
}

impl Display for Leaf {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}")
    }
}

/// The type of widget displayed inside a [`Leaf`].
///
/// # Variants
///
/// - [`Editor`](WidgetType::Editor): The area in which users may write their story
/// - [`Outline`](WidgetType::Outline): A document structure view
/// - [`Notes`](WidgetType::Notes): Author annotations and notes attached to the Flight and/or its Rachises
/// - [`Story`](WidgetType::Story): High-level story metadata and progress tracking
/// - [`Tags`](WidgetType::Tags): An overview of all tags present in the Flight
/// - [`Picker`](WidgetType::Picker): A placeholder shown when no widget type has been selected yet.
/// - [`Empty`](WidgetType::Empty): Displays a picker menu for the user to choose a widget type.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS, Default)]
#[ts(export, repr(enum = name))]
pub enum WidgetType {
    /// The area in which users may write their story
    Editor,
    /// A document structure view
    Outline,
    /// Author annotations and notes attached to the Flight and/or its Rachises
    Notes,
    /// High-level story metadata and progress tracking
    Story,
    /// An overview of all tags present in the Flight
    Tags,
    /// A placeholder shown when no widget type has been selected yet. Displays
    /// a picker menu for the user to choose a widget type.
    #[default]
    Picker,
    /// A debug placeholder; never created by normal user interaction
    Empty,
}

impl Display for WidgetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            WidgetType::Editor => write!(f, "Editor"),
            WidgetType::Outline => write!(f, "Outline"),
            WidgetType::Notes => write!(f, "Notes"),
            WidgetType::Story => write!(f, "Story"),
            WidgetType::Tags => write!(f, "Tags"),
            WidgetType::Picker => write!(f, "Picker"),
            WidgetType::Empty => write!(f, "Empty"),
        }
    }
}

/// The direction along which a [`Branch`] lays out its children.
///
/// # Variants
///
/// - [`Horizontal`](Direction::Horizontal): Children are laid out side-by-side,
/// left to right
/// - [`Vertical`](Direction::Vertical): Children are laid out top-to-bottom
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Default)]
#[ts(export)]
pub enum Direction {
    /// Children are laid out side-by-side, left to right
    #[default]
    Horizontal,
    /// Children are laid out top-to-bottom
    Vertical,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Direction::Horizontal => write!(f, "Horizontal"),
            Direction::Vertical => write!(f, "Vertical"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_type_default() {
        let widget_type: WidgetType = Default::default();
        println!("Default WidgetType: {widget_type:#?}");
        assert_eq!(widget_type, WidgetType::Picker);
    }

    #[test]
    fn test_direction_default() {
        let dir: Direction = Default::default();
        println!("Default Direction: {dir:#?}");
        assert_eq!(dir, Direction::Horizontal);
    }

    #[test]
    fn test_tree_node_creates_branch() {
        let node: TreeNode = TreeNode::Branch(Default::default());
        println!("TreeNode: {node:#?}");
        assert!(matches!(node, TreeNode::Branch(_)));
    }

    #[test]
    fn test_tree_node_creates_leaf() {
        let node: TreeNode = TreeNode::Leaf(Default::default());
        println!("TreeNode: {node:#?}");
        assert!(matches!(node, TreeNode::Leaf(_)));
    }
}

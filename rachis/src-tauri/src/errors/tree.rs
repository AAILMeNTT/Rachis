use {
    crate::tree::TreeNode,
    std::{
        error::Error,
        fmt::{Display, Formatter, Result as FmtResult},
    },
    uuid::Uuid,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreeError {
    /// The [`TreeNode`] was not found in the current `TreeNode`
    NodeNotFound(Uuid),
    /// The [`Branch`] was not found in the current [`TreeNode`]
    BranchNotFound(Uuid),
    /// The [`Leaf`] was not found in the current [`TreeNode`]
    LeafNotFound(Uuid),
    /// The [`TreeNode`] was not found as a child in the current `TreeNode`
    /// If the current `TreeNode` has no children, use [`NoChildren`](TreeError) instead
    ChildNodeNotFound(Uuid),
    /// The [`TreeNode`] has no children
    /// If the current `TreeNode` does have children but the child was not found, use [`ChildNodeNotFound`](TreeError) instead
    NoChildren(TreeNode),
    WrongNodeKind {
        expected: String,
        actual: String
    },
}

impl Display for TreeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TreeError::NodeNotFound(node_id) => write!(f, "Node not found: {node_id}"),
            TreeError::BranchNotFound(branch_id) => write!(f, "Branch not found: {branch_id}"),
            TreeError::LeafNotFound(leaf_id) => write!(f, "Leaf not found: {leaf_id}"),
            TreeError::ChildNodeNotFound(child_id) => write!(f, "Child not found: {child_id}"),
            TreeError::NoChildren(node) => write!(f, "No children: {node}"),
            TreeError::WrongNodeKind { expected, actual } => {
                write!(f, "Wrong node kind: expected {expected}, actual {actual}")
            }
        }
    }
}

impl Error for TreeError {}

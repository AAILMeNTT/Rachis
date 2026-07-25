use {
    crate::tree::{Branch, Leaf, TreeNode, Tree},
    std::{
        error::Error,
        fmt::{Display, Formatter, Result as FmtResult},
    },
    uuid::Uuid,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreeError {
    /// The [`TreeNode`] was not found in the [`Tree`]
    NodeNotFound(Uuid),
    /// The [`Branch`] was not found in the [`Tree`]
    BranchNotFound(Uuid),
    /// The [`Leaf`] was not found in the [`Tree`]
    LeafNotFound(Uuid),
    /// The [`TreeNode`] was not found in the [`TreeNode`]
    ChildNotFound(Uuid),
    /// The [`TreeNode`] has no children
    NoChildren(TreeNode),
    // WrongNodeKind {
    //     expected: &'static str,
    //     actual: &'static str,
    // },
}

impl Display for TreeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TreeError::NodeNotFound(node_id) => write!(f, "Node not found: {}", node_id),
            TreeError::BranchNotFound(id) => write!(f, "Branch not found: {}", id),
            TreeError::LeafNotFound(id) => write!(f, "Leaf not found: {}", id),
            TreeError::ChildNotFound(id) => write!(f, "Child not found: {}", id),
            TreeError::NoChildren(node) => write!(f, "No children: {}", node),
            // TreeError::WrongNodeKind { expected, actual } => write!(
            //     f,
            //     "Wrong node kind: expected {}, actual {}",
            //     expected, actual
            // ),
        }
    }
}

impl Error for TreeError {}

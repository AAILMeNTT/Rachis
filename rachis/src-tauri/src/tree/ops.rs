use {
    crate::tree::{Branch, Direction, Leaf, Tree, TreeNode},
    uuid::Uuid,
};

#[allow(dead_code)]
impl Tree {
    /// Adds a child node to the tree at the specified index.
    ///
    /// # Fields
    ///
    /// - `target_id`: [`Uuid`] The ID of the node to add the child to.
    /// - `child_node`: [`TreeNode`] - The child node to add.
    /// - `index`: [`Option<usize>`](usize) The index at which to insert the child node.
    ///
    /// # Returns
    ///
    /// - `Ok(())` - If the child was added successfully, or an error message if the target node was not found.
    pub fn add_child(
        &mut self,
        target_id: Uuid,
        child_node: TreeNode,
        index: Option<usize>,
    ) -> Result<(), String> {
        // Get a mutable reference to the target node by ID
        let target: &mut TreeNode = Self::find_node_mut(&mut self.root, target_id)
            .ok_or_else(|| "Target not found".to_string())?;

        match target {
            // If the target is a Branch, insert the child node at the specified index or append it
            TreeNode::Branch(b) => match index {
                Some(i) if i <= b.children.len() => b.children.insert(i, child_node),
                Some(_) | None => b.children.push(child_node),
            },
            // If the target is a Leaf, replace it with a Branch containing the old Leaf and the new child
            TreeNode::Leaf(_) => {
                let old_leaf: TreeNode = std::mem::replace(target, TreeNode::Branch(Branch::new()));
                if let TreeNode::Branch(b) = target {
                    b.children.push(old_leaf);
                    b.children.push(child_node);
                }
            }
        }
        Ok(())
    }

    /// Removes a TreeNode from the tree.
    ///
    /// Removing a node from the Tree is potentially state-erroneous, in that
    /// it may result in the violation of two rules:
    ///
    /// 1. A [`Branch`] must never have 0 children; if it does, it must
    /// be removed
    /// 2. A `Branch` must never have 1 child; if it does, it must be
    /// replaced with its child, regardless of if its child is itself a `Branch`
    /// or a [`Leaf`].
    ///
    /// To resolve these error states, `collapse()` is called after removing the
    /// child. See the [`collapse()`](Self::collapse()) function for more detail.
    ///
    /// # Fields
    ///
    /// - `branch_id`: [`Uuid`] - The ID of the [`Branch`] to remove the child from.
    /// - `child_id`: [`Uuid`] - The ID of the child to remove.
    pub fn remove_child(&mut self, branch_id: Uuid, child_id: Uuid) -> Result<TreeNode, String> {
        let target: &mut TreeNode = Self::find_node_mut(&mut self.root, branch_id)
            .ok_or_else(|| "Branch not found".to_string())?;

        match target {
            TreeNode::Branch(b) => {
                // Find the index of the child to remove
                let index: usize = b
                    .children
                    .iter()
                    .position(|c| c.get_id() == child_id)
                    .ok_or_else(|| "Child not found".to_string())?;
                // Remove the child at the found index
                let removed: TreeNode = b.children.remove(index);
                // Begin recursive collapse to remove empty Branches
                self.collapse();

                Ok(removed)
            }
            TreeNode::Leaf(_) => Err("Cannot remove child from a Leaf".to_string()),
        }
    }

    /// Splits a Leaf node into a Branch with two new Leaf children, one in the given direction and one in the opposite direction.
    ///
    /// # Fields
    ///
    /// - `leaf_id`: [`Uuid`] The ID of the Leaf node to split.
    /// - `direction`: [`Direction`] The direction to split the Leaf node in.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: The Leaf node was successfully split into a Branch.
    /// - `Err(String)`: The Leaf node could not be found or split.
    pub fn split_leaf(&mut self, leaf_id: Uuid, direction: Direction) -> Result<(), String> {
        // Get a mutable reference to the node to split
        let target: &mut TreeNode = Self::find_node_mut(&mut self.root, leaf_id)
            .ok_or_else(|| format!("Node with ID {} not found in Tree", leaf_id).to_string())?;

        match target {
            // If the node is a Leaf, turn it into a Branch with it and a new Leaf as children
            TreeNode::Leaf(l) => {
                // Replace the Leaf with a Branch containing the original Leaf and a new Leaf
                *target = TreeNode::Branch(Branch {
                    direction,
                    children: vec![l.as_node(), TreeNode::Leaf(Leaf::default())],
                    ..Branch::default()
                });

                Ok(())
            }
            TreeNode::Branch(_) => {
                Err(format!("ID {} belongs to a Branch, not a Leaf", leaf_id).to_string())
            }
        }
    }

    /// Collapses the workspace tree bottom-up, removing empty Branches (Rule 1),
    /// and flattening singleton Branch chains (Rule 2).
    ///
    /// The cascade is automatic, in that if a child collapse makes its parent
    /// violate one of the two rules, the parent is processed, too. For example,
    /// take this Tree in an error-state:
    ///
    /// ```text
    /// Tree
    /// ├── BranchA
    /// ├── BranchB
    /// │   └── LeafA
    /// └── LeafB
    /// ```
    ///
    /// Errors:
    /// - BranchA has no children (Rule 1 violation), so it is removed.
    /// - BranchB has only one child (Rule 2 violation), so it is replaced with its child.
    ///
    /// After collapsing, the tree becomes:
    ///
    /// ```text
    /// Tree
    /// └── BranchA (new)
    ///     ├── LeafA
    ///     └── LeafB
    /// ```
    ///
    /// Because both LeafA and LeafB would be children of `Tree`, they are wrapped into a new Branch.
    fn collapse(&mut self) {
        let old_root: TreeNode = std::mem::replace(&mut self.root, TreeNode::default());
        // Set the new root as the collapsed node
        match Self::collapse_node(old_root) {
            Some(new_root) => self.root = new_root,
            None => self.root = TreeNode::default(),
        }
    }

    /// Collapses a single node. Returns `Some(node)` if the node should remain,
    /// or `None` if it should be removed from its parent (empty Branch).
    ///
    /// # Fields
    ///
    /// - `node`: [`TreeNode`] - The node to collapse
    ///
    /// # Returns
    ///
    /// - `None` - If `node` is a `Branch` that has, at any point, no children.
    /// - [`Some(TreeNode::Branch)`](Branch) - If `node` is a `Branch` with a
    /// `Branch` as a sole child. If _its_ child is a `Branch`, then `node` will be
    /// replaced with its child.
    /// - [`Some(TreeNode::Branch)`](Branch) - If `node` is a `Branch` with a Leaf as a sole child.
    /// - [`Some(TreeNode::Leaf)`](Leaf) - If `node` is a Leaf.
    fn collapse_node(node: TreeNode) -> Option<TreeNode> {
        match node {
            // If node is a Branch...
            TreeNode::Branch(mut b) => {
                // Recursive call to map each child to its collapsed form
                b.children = b
                    .children
                    .into_iter()
                    // This is the magic part it's so freakin weird just trust me
                    .filter_map(Self::collapse_node)
                    .collect::<Vec<TreeNode>>();

                // Check how many children this Branch has
                match b.children.len() {
                    // Rule 1 violation; empty Branches must be removed
                    0 => None,
                    // Rule 2 violation; single-child Branches must be replaced with their child
                    1 => {
                        // Get the Branch's child
                        let child: TreeNode = b.children.into_iter().next()?;
                        match child {
                            // If the child is itself a Branch, signal to propagate the child to its parent's level
                            TreeNode::Branch(inner) => Some(TreeNode::Branch(inner)),
                            // If the child is a Leaf, the single-child Branch is valid
                            leaf => Some(TreeNode::Branch(Branch {
                                children: vec![leaf],
                                ..b
                            })),
                        }
                    }
                    // Normal case; Branches with >1 child are permitted
                    _ => Some(TreeNode::Branch(b)),
                }
            }
            // If `node` is a Leaf, LEAF IT ALONE!!!!! get it? because they can't be collapsed
            leaf @ TreeNode::Leaf(_) => Some(leaf),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::WidgetType;

    // ========================================================================
    // Helper functions
    // ========================================================================

    /// Helper functions
    fn test_branch() -> Branch {
        Branch {
            id: Uuid::parse_str("10000000-0000-0000-0000-000000000001").unwrap(),
            ..Branch::default()
        }
    }

    fn test_leaf(id_suffix: u8) -> Leaf {
        Leaf {
            id: Uuid::parse_str(&format!(
                "00000000-0000-0000-0000-0000000000{:02x}",
                id_suffix
            ))
            .unwrap(),
            ..Leaf::default()
        }
    }

    // ========================================================================
    // find_branch / find_leaf
    // ========================================================================

    #[test]
    fn test_find_branch_in_tree_with_no_branches() {
        // Instantiate new Tree
        let tree: Tree = Tree::new();
        println!("Tree: {tree:#?}");

        // Generate random Branch uuid
        let branch_id: Uuid = Uuid::new_v4();
        println!("Random Branch ID: {branch_id:#?}");

        // Verify random Branch does not exist
        let result: Result<&Branch, String> = tree.find_branch(&branch_id);
        println!("Result: {result:#?}");
        assert!(result.is_err());
    }

    #[test]
    fn test_find_leaf_in_tree_with_no_leaves() {
        // Instantiate new Tree with a branch root
        let tree: Tree = Tree {
            root: TreeNode::Branch(Branch::default()),
        };
        println!("Tree: {tree:#?}");

        // Generate random Leaf uuid
        let leaf_id: Uuid = Uuid::new_v4();
        println!("Random Leaf ID: {leaf_id:#?}");

        // Verify random Leaf does not exist
        let result: Result<&Leaf, String> = tree.find_leaf(&leaf_id);
        println!("Result: {result:#?}");
        assert!(result.is_err());
    }

    #[test]
    fn test_find_branch_in_tree_with_one_branch() {
        // Instantiate new Workspace
        let tree: Tree = Tree {
            root: TreeNode::Branch(Branch::default()),
        };
        println!("Tree: {tree:#?}");

        // Get branches and verify that only root branch is returned
        let branches: Vec<&Branch> = tree.get_branches();
        println!("Branches: {branches:#?}");
        assert!(branches.len() == 1);

        // Find branch by ID and verify it matches the root branch
        let tree_first_branch: &Branch = tree.find_branch(&branches[0].id).unwrap();
        println!("First branch: {tree_first_branch:#?}");
        assert!(branches[0].id == tree_first_branch.id);
    }

    #[test]
    fn test_find_leaf_in_tree_with_one_leaf() {
        // Instantiate new Tree
        let tree: Tree = Tree::new();
        println!("Tree: {tree:#?}");

        // Get leaves and verify that only root leaf is returned
        let leaves: Vec<&Leaf> = tree.get_leaves();
        println!("Leaves: {leaves:#?}");
        assert!(leaves.len() == 1);

        // Find leaf by ID and verify it matches the root leaf
        let tree_first_leaf: &Leaf = tree.find_leaf(&leaves[0].id).unwrap();
        println!("First leaf: {tree_first_leaf:#?}");
        assert!(leaves[0].id == tree_first_leaf.id);
    }

    #[test]
    fn test_find_branch_in_tree_with_many_branches() {
        // Create empty Branch
        let branch: Branch = test_branch();
        println!("Branch to find: {branch:#?}");

        // Instantiate new Tree with many branches
        let tree: Tree = Tree {
            root: TreeNode::Branch(Branch {
                children: vec![
                    // Leaf node
                    TreeNode::Leaf(Leaf::default()),
                    // Branch (empty)
                    TreeNode::Branch(branch.clone()),
                    // Branch (one child)
                    TreeNode::Branch(Branch {
                        children: vec![TreeNode::Leaf(Leaf::default())],
                        ..Branch::default()
                    }),
                    // Branch (many children)
                    TreeNode::Branch(Branch {
                        children: vec![
                            TreeNode::Leaf(Leaf::default()),
                            TreeNode::Leaf(Leaf::default()),
                            TreeNode::Leaf(Leaf::default()),
                        ],
                        ..Branch::default()
                    }),
                ],
                ..Branch::default()
            }),
        };
        // println!("Tree: {tree:#?}");

        let branches: Vec<&Branch> = tree.get_branches();
        // Feel free to uncomment if you're brave
        // (Maybe... could it be useful for debugging/maintenance to add a "display()" func to visualise what a Tree/TreeNode looks like?)
        // println!("Branches: {branches:#?}");
        println!("Branch count: {:?}", branches.len());
        assert!(branches.len() == 4);

        // Verify that the found branch is the same as the original branch made
        let found_branch: &Branch = tree.find_branch(&branch.id).unwrap();
        println!("Found Branch: {found_branch:#?}");
        assert_eq!(found_branch.id, branch.id);
    }

    #[test]
    fn test_find_leaf_in_tree_with_many_leaves() {
        // Create empty Branch
        let leaf: Leaf = test_leaf(1);
        println!("Leaf to find: {leaf:#?}");

        // Instantiate new Tree with many branches
        let tree: Tree = Tree {
            root: TreeNode::Branch(Branch {
                children: vec![
                    // Leaf node
                    TreeNode::Leaf(leaf.clone()),
                    // Branch (empty)
                    TreeNode::Branch(Branch::default()),
                    // Branch (one child)
                    TreeNode::Branch(Branch {
                        children: vec![TreeNode::Leaf(Leaf::default())],
                        ..Branch::default()
                    }),
                    // Branch (many children)
                    TreeNode::Branch(Branch {
                        children: vec![
                            TreeNode::Leaf(Leaf::default()),
                            TreeNode::Leaf(Leaf::default()),
                            TreeNode::Leaf(Leaf::default()),
                        ],
                        ..Branch::default()
                    }),
                ],
                ..Branch::default()
            }),
        };
        // println!("Tree: {tree:#?}");

        let leaves: Vec<&Leaf> = tree.get_leaves();
        println!("Leaves: {leaves:#?}");
        println!("Leaf count: {:?}", leaves.len());
        assert!(leaves.len() == 5);

        // Verify that the found Leaf is the same as the original Leaf made
        let found_leaf: &Leaf = tree.find_leaf(&leaf.id).unwrap();
        println!("Found Branch: {found_leaf:#?}");
        assert_eq!(found_leaf.id, leaf.id);
    }

    // ========================================================================
    // add_child
    // ========================================================================

    /// Test adding a Leaf to a Tree whose root node is a Leaf
    #[test]
    fn test_add_leaf_to_tree_with_leaf_as_root() {
        // Instantiate new mutable Tree
        let mut tree: Tree = Tree::new();
        println!("Tree: {tree:#?}");
        // Create a test Leaf to add as a child
        let leaf_to_add: Leaf = test_leaf(1);
        println!("Leaf: {leaf_to_add:#?}");

        // Get the initial tree's root ID (a Picker Leaf)
        let leaves_before: Vec<Uuid> = tree
            .get_leaves()
            .iter()
            .map(|l| l.id)
            .collect::<Vec<Uuid>>();
        println!("Leaves before addition: {leaves_before:#?}");
        assert_eq!(leaves_before.len(), 1);
        let target_id: Uuid = leaves_before[0];
        println!("Target Leaf: {target_id:#?}");

        // Add a new Leaf as a child
        // Since target is a Leaf, add_child should wrap it in a Branch first
        tree.add_child(target_id, leaf_to_add.as_node(), None)
            .unwrap();
        println!("Tree after addition: {tree:#?}");

        // The tree should now have a Branch as root (was a Leaf, got wrapped)
        let branches: Vec<&Branch> = tree.get_branches();
        println!("Branch count: {:?}", branches.len());
        assert_eq!(branches.len(), 1);

        let leaves: Vec<&Leaf> = tree.get_leaves();
        println!("Leaves: {leaves:#?}");
        println!("Leaf count: {:?}", leaves.len());
        assert_eq!(leaves.len(), 2);
    }

    /// Test adding multiple leaves to a Tree with an empty Branch as the root
    #[test]
    fn test_add_multiple_leaves_to_tree_with_empty_branch_as_root() {
        // Instantiate new Tree with a Branch root
        let mut tree: Tree = Tree {
            root: TreeNode::Branch(test_branch()),
        };
        println!("Tree: {tree:#?}");
        // Create Leaves to add
        let leaf_a: TreeNode = test_leaf(1).as_node();
        let leaf_b: TreeNode = test_leaf(2).as_node();
        println!("Leaf a: {leaf_a:#?}");
        println!("Leaf b: {leaf_b:#?}");

        let branch_id: Uuid = tree.root.get_id();
        println!("Branch ID: {branch_id:#?}");

        // Add two children
        tree.add_child(branch_id, leaf_a, None).unwrap();
        tree.add_child(branch_id, leaf_b, None).unwrap();
        println!("Tree after additions: {tree:#?}");

        let leaves: Vec<&Leaf> = tree.get_leaves();
        println!("Leaves: {leaves:#?}");
        assert_eq!(leaves.len(), 2);
    }

    /// Test adding a Leaf to a Branch at a specific index
    #[test]
    fn test_add_child_to_branch_at_index() {
        // Instantiate a Tree with an empty Branch as the root
        let mut tree: Tree = Tree {
            root: TreeNode::Branch(test_branch()),
        };
        println!("Tree: {tree:#?}");
        // Get the Branch ID of the root Branch
        let branch_id: Uuid = tree.root.get_id();
        println!("Branch ID: {branch_id:#?}");

        // Create test Leaf nodes
        let leaf_a: TreeNode = test_leaf(1).as_node();
        let leaf_b: TreeNode = test_leaf(2).as_node();
        let leaf_c: TreeNode = test_leaf(3).as_node();
        println!("Leaf A: {leaf_a:#?}");
        println!("Leaf B: {leaf_b:#?}");
        println!("Leaf C: {leaf_c:#?}");

        // Insert A and B, then insert C at index 1 (between them)
        tree.add_child(branch_id, leaf_a, None).unwrap();
        tree.add_child(branch_id, leaf_b, None).unwrap();
        tree.add_child(branch_id, leaf_c, Some(1)).unwrap();

        // Verify the order by checking positions
        let branch: &Branch = tree.find_branch(&branch_id).unwrap();
        println!("Branch: {branch:#?}");
        assert_eq!(branch.children.len(), 3);

        // Child at index 1 should be C (id suffix 0x03)
        let child_at_1: &TreeNode = &branch.children[1];
        println!("Child at index 1: {:?}", child_at_1);
        assert_eq!(child_at_1.get_id(), test_leaf(3).id);
    }

    /// Test adding a child to a non-existent target in the tree
    #[test]
    fn test_add_child_target_not_found() {
        let mut tree: Tree = Tree::new();
        println!("Tree before add: {tree:#?}");

        let leaf: TreeNode = test_leaf(1).as_node();
        println!("Leaf to add: {leaf:#?}");

        let result: Result<(), String> = tree.add_child(Uuid::new_v4(), leaf, None);
        println!("Tree after add: {tree:#?}");
        println!("Result: {result:#?}");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_child_wraps_leaf() {
        // Adding to a Leaf should wrap it in a Branch
        let mut tree: Tree = Tree::new();
        println!("Tree before add: {tree:#?}");
        let original_leaf_id: Uuid = tree.get_leaves()[0].id;
        println!("Original leaf id: {original_leaf_id:#?}");
        let new_leaf: TreeNode = test_leaf(1).as_node();
        println!("New leaf: {new_leaf:#?}");

        tree.add_child(original_leaf_id, new_leaf, None).unwrap();
        println!("Tree after add: {tree:#?}");

        // Root should now be a Branch wrapping both leaves
        match &tree.root {
            TreeNode::Branch(b) => {
                println!("Root node is a Branch");
                println!("Branch children count: {:?}", b.children.len());
                assert_eq!(b.children.len(), 2);
                println!("Branch children: {:?}", b.children);
                assert!(b.children.iter().any(|c| c.get_id() == original_leaf_id));
            }
            TreeNode::Leaf(_) => panic!("Expected root to be a Branch after wrapping Leaf"),
        }
    }

    // ========================================================================
    // remove_child
    // ========================================================================

    #[test]
    fn test_remove_child() {
        let mut tree: Tree = Tree {
            root: TreeNode::Branch(test_branch()),
        };
        println!("Tree: {tree:#?}");
        let branch_id: Uuid = tree.get_branches()[0].id;
        println!("Branch ID: {branch_id:#?}");

        let leaf: TreeNode = TreeNode::Leaf(Leaf {
            id: Uuid::parse_str("00000000-0000-0000-0000-111111111111").unwrap(),
            ..Leaf::default()
        });
        println!("Leaf: {leaf:#?}");
        tree.add_child(branch_id, leaf, None).unwrap();
        println!("Tree after add: {tree:#?}");

        let child_id: Uuid = tree.get_leaves()[0].id;
        println!("Child ID: {child_id:#?}");
        let removed: TreeNode = tree.remove_child(branch_id, child_id).unwrap();
        println!("Tree after remove: {tree:#?}");

        // Should have removed the correct node
        assert_eq!(removed.get_id(), child_id);

        // remove_child() collapses the branch into a single PickerWidget Leaf,
        let leaves: Vec<&Leaf> = tree.get_leaves();
        assert_eq!(leaves.len(), 1);
        assert_eq!(leaves[0].widget_type, WidgetType::Picker);
    }

    #[test]
    fn test_remove_child_nonexistent() {
        let mut tree: Tree = Tree {
            root: TreeNode::Branch(test_branch()),
        };
        let branch_id = tree.get_branches()[0].id;

        let result: Result<TreeNode, String> = tree.remove_child(branch_id, Uuid::new_v4());
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_child_from_leaf() {
        let mut tree: Tree = Tree::new();
        let leaf_id: Uuid = tree.get_leaves()[0].id;

        let result: Result<TreeNode, String> = tree.remove_child(leaf_id, Uuid::new_v4());
        assert!(result.is_err());
    }

    // ========================================================================
    // split_leaf
    // ========================================================================

    #[test]
    fn test_split_leaf() {
        // Use a non-Picker leaf so we can distinguish original from new
        let editor_leaf: Leaf = Leaf {
            widget_type: WidgetType::Editor,
            ..Leaf::default()
        };
        let mut tree: Tree = Tree {
            root: TreeNode::Leaf(editor_leaf),
        };
        let leaf_id: Uuid = tree.get_leaves()[0].id;

        tree.split_leaf(leaf_id, Direction::Horizontal).unwrap();

        // Root should now be a Branch with 2 children (the original + a Picker)
        match &tree.root {
            TreeNode::Branch(b) => {
                assert_eq!(b.children.len(), 2);
                assert_eq!(b.direction, Direction::Horizontal);

                // One child should be the original Editor leaf
                let editor_count: usize = b
                    .children
                    .iter()
                    .filter(|c| {
                        c.as_leaf()
                            .map_or(false, |l| l.widget_type == WidgetType::Editor)
                    })
                    .count();
                assert_eq!(editor_count, 1);

                // The other should be a Picker
                let picker_count: usize = b
                    .children
                    .iter()
                    .filter(|c| {
                        c.as_leaf()
                            .map_or(false, |l| l.widget_type == WidgetType::Picker)
                    })
                    .count();
                assert_eq!(picker_count, 1);
            }
            TreeNode::Leaf(_) => panic!("Expected root to be a Branch after split"),
        }
    }

    #[test]
    fn test_split_leaf_vertical() {
        let mut tree: Tree = Tree::new();
        let leaf_id: Uuid = tree.get_leaves()[0].id;

        tree.split_leaf(leaf_id, Direction::Vertical).unwrap();

        match &tree.root {
            TreeNode::Branch(b) => {
                assert_eq!(b.direction, Direction::Vertical);
                assert_eq!(b.children.len(), 2);
            }
            TreeNode::Leaf(_) => panic!("Expected root to be a Branch after split"),
        }
    }

    #[test]
    fn test_split_branch_errors() {
        let mut tree: Tree = Tree {
            root: TreeNode::Branch(Branch::default()),
        };
        let branch_id = tree.get_branches()[0].id;

        let result = tree.split_leaf(branch_id, Direction::Horizontal);
        assert!(result.is_err());
    }

    #[test]
    fn test_split_leaf_nonexistent() {
        let mut tree: Tree = Tree::new();
        let result = tree.split_leaf(Uuid::new_v4(), Direction::Horizontal);
        assert!(result.is_err());
    }

    // ========================================================================
    // collapse (tested implicitly through remove_child + custom trees)
    // ========================================================================

    #[test]
    fn test_collapse_empty_branch_removed() {
        // A Branch with 0 children should be removed by collapse
        let mut tree: Tree = Tree {
            root: TreeNode::Branch(test_branch()),
        };

        // The root Branch is empty. Collapse should remove it
        // and replace with a default Leaf.
        tree.collapse();

        // Root should now be a Leaf (Picker)
        match &tree.root {
            TreeNode::Leaf(l) => assert_eq!(l.widget_type, WidgetType::Picker),
            TreeNode::Branch(_) => panic!("Expected empty Branch to be collapsed"),
        }
    }

    #[test]
    fn test_collapse_singleton_branch_chain() {
        // Outer Branch containing inner Branch containing a Leaf
        let leaf: TreeNode = test_leaf(1).as_node();
        let inner_branch: TreeNode = TreeNode::Branch(Branch {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            direction: Direction::Vertical,
            children: vec![leaf],
            ratios: vec![],
        });
        let tree: Tree = Tree {
            root: TreeNode::Branch(Branch {
                id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
                direction: Direction::Horizontal,
                children: vec![inner_branch],
                ratios: vec![],
            }),
        };

        // Let mut collapse work on it
        let mut tree = tree;
        tree.collapse();

        // The outer Branch should have been collapsed away:
        // chain was Branch(H) → Branch(V) → Leaf
        // should become Branch(V) → Leaf
        match &tree.root {
            TreeNode::Branch(b) => {
                assert_eq!(
                    b.id,
                    Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()
                );
                assert_eq!(b.direction, Direction::Vertical);
                assert_eq!(b.children.len(), 1);
            }
            _ => panic!("Expected root to remain a Branch"),
        }
    }

    #[test]
    fn test_collapse_singleton_leaf_kept() {
        // A Branch with a single Leaf child should keep the Branch
        let leaf: TreeNode = test_leaf(1).as_node();
        let mut tree: Tree = Tree {
            root: TreeNode::Branch(Branch {
                id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
                direction: Direction::Horizontal,
                children: vec![leaf],
                ratios: vec![],
            }),
        };

        tree.collapse();

        // Branch with single Leaf should be kept
        match &tree.root {
            TreeNode::Branch(b) => {
                assert_eq!(b.children.len(), 1);
                assert!(b.children[0].is_leaf());
            }
            _ => panic!("Expected root to remain a Branch"),
        }
    }

    #[test]
    fn test_collapse_multi_child_branch_kept() {
        let leaf_a: TreeNode = test_leaf(1).as_node();
        let leaf_b: TreeNode = test_leaf(2).as_node();
        let mut tree: Tree = Tree {
            root: TreeNode::Branch(Branch {
                id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
                direction: Direction::Horizontal,
                children: vec![leaf_a, leaf_b],
                ratios: vec![],
            }),
        };

        tree.collapse();

        // Branch with 2 children should remain unchanged
        match &tree.root {
            TreeNode::Branch(b) => {
                assert_eq!(b.children.len(), 2);
            }
            _ => panic!("Expected root to remain a Branch"),
        }
    }

    // ========================================================================
    // combined operations
    // ========================================================================

    #[test]
    fn test_add_then_remove_returns_to_singleton_branch() {
        let mut tree: Tree = Tree::new();
        let original_id = tree.get_leaves()[0].id;

        // Add a child (wraps original Leaf in Branch)
        let new_leaf: TreeNode = test_leaf(1).as_node();
        tree.add_child(original_id, new_leaf, None).unwrap();

        // Now we have a Branch with 2 children.
        // Remove the new one. This should trigger collapse.
        let branch_id = tree.get_branches()[0].id;
        let child_id = tree
            .get_leaves()
            .iter()
            .find(|l| l.id != original_id)
            .unwrap()
            .id;

        let _ = tree.remove_child(branch_id, child_id).unwrap();

        // The Branch has 1 child (original Leaf).
        // Singleton Leaf Branches are VALID — the renderer shows
        // them at full size with no chrome. No further collapse.
        match &tree.root {
            TreeNode::Branch(b) => {
                assert_eq!(b.children.len(), 1);
                assert!(b.children[0].is_leaf());
                assert_eq!(b.children[0].get_id(), original_id);
            }
            _ => panic!("Expected a Branch with singleton Leaf root"),
        }
    }

    #[test]
    fn test_complex_tree_add_remove() {
        // Build: Branch(H) → [Leaf A, Branch(V) → [Leaf B, Leaf C]]
        let leaf_a: TreeNode = test_leaf(1).as_node();
        let leaf_b: TreeNode = test_leaf(2).as_node();
        let leaf_c: TreeNode = test_leaf(3).as_node();

        let inner_branch_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
        let inner_branch: TreeNode = TreeNode::Branch(Branch {
            id: inner_branch_id,
            direction: Direction::Vertical,
            children: vec![leaf_b, leaf_c],
            ratios: vec![],
        });

        let outer_branch_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        let mut tree: Tree = Tree {
            root: TreeNode::Branch(Branch {
                id: outer_branch_id,
                direction: Direction::Horizontal,
                children: vec![leaf_a, inner_branch],
                ratios: vec![],
            }),
        };

        assert_eq!(tree.get_leaves().len(), 3);
        assert_eq!(tree.get_branches().len(), 2);

        // Remove Leaf B from inner Branch → inner has 1 child (Leaf C)
        let removed = tree.remove_child(inner_branch_id, test_leaf(2).id).unwrap();
        assert_eq!(removed.get_id(), test_leaf(2).id);

        // Inner Branch now has 1 child (Leaf C) → collapse keeps Branch(V)
        // Outer still has 2 children → stays
        assert_eq!(tree.get_leaves().len(), 2);
        assert_eq!(tree.get_branches().len(), 2);
    }

    #[test]
    fn test_remove_cascades_upward() {
        // Build: Branch(outer) → Branch(inner) → Leaf
        let leaf: TreeNode = test_leaf(1).as_node();

        let inner_branch_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
        let inner_branch: TreeNode = TreeNode::Branch(Branch {
            id: inner_branch_id,
            direction: Direction::Vertical,
            children: vec![leaf],
            ratios: vec![],
        });

        let outer_branch_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        let mut tree: Tree = Tree {
            root: TreeNode::Branch(Branch {
                id: outer_branch_id,
                direction: Direction::Horizontal,
                children: vec![inner_branch],
                ratios: vec![],
            }),
        };

        // Remove the sole leaf from inner Branch.
        // Inner becomes empty → collapsed to None → removed from outer.
        // Outer now has 0 children → collapsed to None → replaced with default Leaf.
        let _ = tree.remove_child(inner_branch_id, test_leaf(1).id).unwrap();

        // The entire tree should collapse down to a single Picker Leaf
        match &tree.root {
            TreeNode::Leaf(l) => {
                assert_eq!(l.widget_type, WidgetType::Picker);
            }
            TreeNode::Branch(_) => {
                panic!("Expected full tree to collapse to a single Leaf");
            }
        }
    }

    #[test]
    fn test_remove_branch_child_then_remaining_leaf_persists() {
        // Build: Branch(H) → [Leaf A, Leaf B]
        let leaf_a: TreeNode = test_leaf(1).as_node();
        let leaf_b: TreeNode = test_leaf(2).as_node();

        let branch_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        let mut tree: Tree = Tree {
            root: TreeNode::Branch(Branch {
                id: branch_id,
                direction: Direction::Horizontal,
                children: vec![leaf_a, leaf_b],
                ratios: vec![],
            }),
        };

        // Remove Leaf A → Branch now has 1 child (Leaf B)
        // Collapse keeps Branch with singleton Leaf (valid state)
        let _ = tree.remove_child(branch_id, test_leaf(1).id).unwrap();

        // Should still have a Branch with one Leaf child
        match &tree.root {
            TreeNode::Branch(b) => {
                assert_eq!(b.children.len(), 1);
                assert!(b.children[0].is_leaf());
            }
            _ => panic!("Expected a Branch with singleton Leaf"),
        }
    }
}

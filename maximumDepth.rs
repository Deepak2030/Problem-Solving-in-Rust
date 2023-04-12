use std::cmp;

// Define a binary tree node
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Function to find the maximum depth of a binary tree
fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.left.as_ref().map(|node| node.as_ref()));
            let right_depth = max_depth(node.right.as_ref().map(|node| node.as_ref()));
            cmp::max(left_depth, right_depth) + 1
        }
    }
}

// Example usage
fn main() {
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            })),
        })),
    }));
    let depth = max_depth(root.as_ref());
    println!("The maximum depth of the binary tree is {}", depth);
}

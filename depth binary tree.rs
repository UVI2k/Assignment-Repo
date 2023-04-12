struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.left.as_ref());
            let right_depth = max_depth(node.right.as_ref());
            1 + std::cmp::max(left_depth, right_depth)
        }
    }
}

fn main() {
    // Example binary tree
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Box::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            })),
        })),
    }));
    let depth = max_depth(root.as_ref());
    println!("The maximum depth of the tree is {}", depth);
}

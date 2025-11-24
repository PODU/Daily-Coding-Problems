// Validate BST with inclusive bounds: left<=node, right>=node (duplicates allowed both sides).
// Recursive (low,high) bound check. Time O(n), Space O(h). fn main.

struct TreeNode {
    val: i64,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i64) -> Box<TreeNode> {
        Box::new(TreeNode { val, left: None, right: None })
    }
}

fn is_valid(node: &Option<Box<TreeNode>>, low: i64, high: i64) -> bool {
    match node {
        None => true,
        Some(n) => {
            if n.val < low || n.val > high {
                return false;
            }
            is_valid(&n.left, low, n.val) && is_valid(&n.right, n.val, high)
        }
    }
}

fn is_bst(root: &Option<Box<TreeNode>>) -> bool {
    is_valid(root, i64::MIN, i64::MAX)
}

fn main() {
    // Tree A (valid): root=5, left=3(l=2,r=5), right=8(l=8,r=9)
    let mut a = TreeNode::new(5);
    let mut a_left = TreeNode::new(3);
    a_left.left = Some(TreeNode::new(2));
    a_left.right = Some(TreeNode::new(5));
    let mut a_right = TreeNode::new(8);
    a_right.left = Some(TreeNode::new(8));
    a_right.right = Some(TreeNode::new(9));
    a.left = Some(a_left);
    a.right = Some(a_right);
    let a = Some(a);

    // Tree B (invalid): root=5, left=3, right=4
    let mut b = TreeNode::new(5);
    b.left = Some(TreeNode::new(3));
    b.right = Some(TreeNode::new(4));
    let b = Some(b);

    println!("{}", is_bst(&a));
    println!("{}", is_bst(&b));
}

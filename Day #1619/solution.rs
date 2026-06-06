// Subtree check via serialization with sentinels + substring search.
// Time: O(n+m), Space: O(n+m).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode { val, left: None, right: None }))
    }
}

fn serialize(node: &Link, out: &mut String) {
    match node {
        None => out.push_str(",#"),
        Some(n) => {
            let n = n.borrow();
            out.push(',');
            out.push_str(&n.val.to_string());
            serialize(&n.left, out);
            serialize(&n.right, out);
        }
    }
}

fn is_subtree(s: &Link, t: &Link) -> bool {
    let mut ss = String::new();
    let mut ts = String::new();
    serialize(s, &mut ss);
    serialize(t, &mut ts);
    ss.contains(&ts)
}

fn main() {
    let s = TreeNode::new(3);
    let s4 = TreeNode::new(4);
    let s5 = TreeNode::new(5);
    s4.borrow_mut().left = Some(TreeNode::new(1));
    s4.borrow_mut().right = Some(TreeNode::new(2));
    s.borrow_mut().left = Some(s4);
    s.borrow_mut().right = Some(s5);

    let t = TreeNode::new(4);
    t.borrow_mut().left = Some(TreeNode::new(1));
    t.borrow_mut().right = Some(TreeNode::new(2));

    let s_link: Link = Some(s);
    let t_link: Link = Some(t);

    println!("{}", if is_subtree(&s_link, &t_link) { "true" } else { "false" });
}

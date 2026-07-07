// Morris in-order traversal via raw pointers: thread tree using predecessor links, O(1) space.
// Time O(N), Space O(1) (excluding output).
struct Node {
    val: i32,
    left: *mut Node,
    right: *mut Node,
}

impl Node {
    fn new(val: i32) -> *mut Node {
        Box::into_raw(Box::new(Node { val, left: std::ptr::null_mut(), right: std::ptr::null_mut() }))
    }
}

fn morris_inorder(root: *mut Node) {
    let mut out: Vec<String> = Vec::new();
    let mut cur = root;
    unsafe {
        while !cur.is_null() {
            if (*cur).left.is_null() {
                out.push((*cur).val.to_string());
                cur = (*cur).right;
            } else {
                let mut pre = (*cur).left;
                while !(*pre).right.is_null() && (*pre).right != cur {
                    pre = (*pre).right;
                }
                if (*pre).right.is_null() {
                    (*pre).right = cur;
                    cur = (*cur).left;
                } else {
                    (*pre).right = std::ptr::null_mut();
                    out.push((*cur).val.to_string());
                    cur = (*cur).right;
                }
            }
        }
    }
    println!("{}", out.join(" "));
}

fn main() {
    let root = Node::new(4);
    unsafe {
        (*root).left = Node::new(2);
        (*root).right = Node::new(5);
        (*(*root).left).left = Node::new(1);
        (*(*root).left).right = Node::new(3);
    }
    morris_inorder(root);
}

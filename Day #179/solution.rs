// Reconstruct BST from postorder: iterate right-to-left as (root,right,left) with a lower bound. O(n) time/space.
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct Builder {
    post: Vec<i32>,
    idx: i32,
}

impl Builder {
    fn build(&mut self, lower: i32) -> Option<Box<Node>> {
        if self.idx < 0 || self.post[self.idx as usize] < lower {
            return None;
        }
        let val = self.post[self.idx as usize];
        self.idx -= 1;
        let right = self.build(val);
        let left = self.build(lower);
        Some(Box::new(Node { val, left, right }))
    }
}

fn inorder(n: &Option<Box<Node>>, o: &mut Vec<i32>) {
    if let Some(b) = n {
        inorder(&b.left, o);
        o.push(b.val);
        inorder(&b.right, o);
    }
}

fn postorder(n: &Option<Box<Node>>, o: &mut Vec<i32>) {
    if let Some(b) = n {
        postorder(&b.left, o);
        postorder(&b.right, o);
        o.push(b.val);
    }
}

fn main() {
    let post = vec![2, 4, 3, 8, 7, 5];
    let n = post.len() as i32;
    let mut b = Builder { post, idx: n - 1 };
    let root = b.build(i32::MIN);
    let mut ino = Vec::new();
    inorder(&root, &mut ino);
    let mut po = Vec::new();
    postorder(&root, &mut po);
    let ino_s: Vec<String> = ino.iter().map(|x| x.to_string()).collect();
    let po_s: Vec<String> = po.iter().map(|x| x.to_string()).collect();
    println!("Inorder: {}", ino_s.join(" "));
    println!("Postorder: {}", po_s.join(" "));
}

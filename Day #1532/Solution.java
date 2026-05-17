// Invert (mirror) a binary tree by swapping left/right children of every node.
// Time O(n), Space O(h) recursion.
public class Solution {
    static class Node {
        char val;
        Node l, r;
        Node(char v) { val = v; }
    }

    static Node invert(Node root) {
        if (root == null) return null;
        Node t = root.l;
        root.l = root.r;
        root.r = t;
        invert(root.l);
        invert(root.r);
        return root;
    }

    static void preorder(Node root, StringBuilder sb) {
        if (root == null) return;
        if (sb.length() > 0) sb.append(' ');
        sb.append(root.val);
        preorder(root.l, sb);
        preorder(root.r, sb);
    }

    public static void main(String[] args) {
        Node a = new Node('a');
        a.l = new Node('b');
        a.r = new Node('c');
        a.l.l = new Node('d');
        a.l.r = new Node('e');
        a.r.l = new Node('f');
        StringBuilder before = new StringBuilder(), after = new StringBuilder();
        preorder(a, before);
        invert(a);
        preorder(a, after);
        System.out.println("before (preorder): " + before);
        System.out.println("after  (preorder): " + after);
    }
}

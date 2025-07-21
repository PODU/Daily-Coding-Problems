// Count unival subtrees: postorder; a subtree is unival if both children are
// unival and match the node's value. Time: O(n), Space: O(h).
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
        Node(int v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static int count = 0;
    static boolean isUnival(Node n) {
        if (n == null) return true;
        boolean l = isUnival(n.left), r = isUnival(n.right);
        if (!l || !r) return false;
        if (n.left != null && n.left.val != n.val) return false;
        if (n.right != null && n.right.val != n.val) return false;
        count++;
        return true;
    }

    public static void main(String[] args) {
        Node root = new Node(0,
            new Node(1),
            new Node(0, new Node(1, new Node(1), new Node(1)), new Node(0)));
        isUnival(root);
        System.out.println(count);
    }
}

// Reconstruct BST from postorder. Process postorder from the right with an
// upper-bound recursion (reverse postorder = root,right,left). Time O(n), Space O(h).
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static int idx;

    static Node build(int[] post, long bound) {
        if (idx < 0 || post[idx] < bound) return null;
        Node root = new Node(post[idx--]);
        root.right = build(post, root.val);
        root.left = build(post, bound);
        return root;
    }

    static Node bstFromPostorder(int[] post) {
        idx = post.length - 1;
        return build(post, Long.MIN_VALUE);
    }

    static void preorder(Node root, List<Integer> out) {
        if (root == null) return;
        out.add(root.val);
        preorder(root.left, out);
        preorder(root.right, out);
    }

    public static void main(String[] args) {
        int[] post = {2, 4, 3, 8, 7, 5};
        Node root = bstFromPostorder(post);
        List<Integer> pre = new ArrayList<>();
        preorder(root, pre);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < pre.size(); i++) {
            sb.append(pre.get(i));
            if (i + 1 < pre.size()) sb.append(' ');
        }
        System.out.println(sb.toString());
    }
}

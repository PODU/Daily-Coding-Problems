// Day 435: Reconstruct a binary tree from preorder + inorder traversals.
// Approach: recurse, using a HashMap of inorder value->index to find roots in O(1).
// Time: O(n), Space: O(n).
import java.util.*;

public class Solution {
    static class Node {
        String val;
        Node left, right;
        Node(String v) { val = v; }
    }

    static int pi = 0;

    static Node build(String[] pre, int inL, int inR, Map<String,Integer> idx) {
        if (inL > inR) return null;
        String rootVal = pre[pi++];
        Node root = new Node(rootVal);
        int mid = idx.get(rootVal);
        root.left = build(pre, inL, mid - 1, idx);
        root.right = build(pre, mid + 1, inR, idx);
        return root;
    }

    public static void main(String[] args) {
        String[] preorder = {"a","b","d","e","c","f","g"};
        String[] inorder  = {"d","b","e","a","f","c","g"};
        Map<String,Integer> idx = new HashMap<>();
        for (int i = 0; i < inorder.length; i++) idx.put(inorder[i], i);
        Node root = build(preorder, 0, inorder.length - 1, idx);

        // Print level-order: a b c d e f g
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        StringBuilder sb = new StringBuilder();
        while (!q.isEmpty()) {
            Node n = q.poll();
            if (sb.length() > 0) sb.append(" ");
            sb.append(n.val);
            if (n.left != null) q.add(n.left);
            if (n.right != null) q.add(n.right);
        }
        System.out.println(sb.toString());
    }
}

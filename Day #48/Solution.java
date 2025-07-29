// Day 48: Reconstruct binary tree from preorder + inorder.
// Hashmap of inorder positions; recurse. Time: O(n), Space: O(n).
import java.util.*;

public class Solution {
    static class Node {
        char val; Node left, right;
        Node(char v) { val = v; }
    }

    static int preIdx = 0;

    static Node build(char[] pre, int inL, int inR, Map<Character,Integer> pos) {
        if (inL > inR) return null;
        char rootVal = pre[preIdx++];
        Node root = new Node(rootVal);
        int mid = pos.get(rootVal);
        root.left = build(pre, inL, mid - 1, pos);
        root.right = build(pre, mid + 1, inR, pos);
        return root;
    }

    public static void main(String[] args) {
        char[] pre = {'a','b','d','e','c','f','g'};
        char[] in  = {'d','b','e','a','f','c','g'};
        Map<Character,Integer> pos = new HashMap<>();
        for (int i = 0; i < in.length; i++) pos.put(in[i], i);
        Node root = build(pre, 0, in.length - 1, pos);

        // Level-order traversal confirms reconstruction: a b c d e f g
        StringBuilder sb = new StringBuilder();
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        while (!q.isEmpty()) {
            Node n = q.poll();
            if (sb.length() > 0) sb.append(' ');
            sb.append(n.val);
            if (n.left != null) q.add(n.left);
            if (n.right != null) q.add(n.right);
        }
        System.out.println(sb);
    }
}

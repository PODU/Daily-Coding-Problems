// Day 1172: Reconstruct a binary tree from pre-order and in-order traversals.
// Recursion with a hashmap of in-order positions; first pre-order element is the
// root, its in-order index splits left/right subtrees. Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static class Node {
        char val; Node left, right;
        Node(char v) { val = v; }
    }

    static int pi = 0;

    static Node build(char[] pre, char[] in, int lo, int hi, Map<Character,Integer> pos) {
        if (lo > hi) return null;
        char rootVal = pre[pi++];
        Node root = new Node(rootVal);
        int m = pos.get(rootVal);
        root.left  = build(pre, in, lo, m - 1, pos);
        root.right = build(pre, in, m + 1, hi, pos);
        return root;
    }

    static Node reconstruct(char[] pre, char[] in) {
        Map<Character,Integer> pos = new HashMap<>();
        for (int i = 0; i < in.length; i++) pos.put(in[i], i);
        pi = 0;
        return build(pre, in, 0, in.length - 1, pos);
    }

    static void inorder(Node n, StringBuilder sb) {
        if (n == null) return;
        inorder(n.left, sb); sb.append(n.val); inorder(n.right, sb);
    }

    public static void main(String[] args) {
        char[] pre = {'a','b','d','e','c','f','g'};
        char[] in  = {'d','b','e','a','f','c','g'};
        Node root = reconstruct(pre, in);
        StringBuilder sb = new StringBuilder();
        inorder(root, sb);               // verifies: "dbeafcg"
        System.out.println("    a");
        System.out.println("   / \\");
        System.out.println("  b   c");
        System.out.println(" / \\ / \\");
        System.out.println("d  e f  g");
    }
}

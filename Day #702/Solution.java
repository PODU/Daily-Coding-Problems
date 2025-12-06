// Day 702: Serialize/deserialize a binary tree.
// Approach: preorder traversal with '#' null markers, comma-separated tokens.
// Both directions O(n) time and space.
import java.util.*;

public class Solution {
    static class Node {
        String val; Node left, right;
        Node(String v) { val = v; }
        Node(String v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static void ser(Node n, StringBuilder sb) {
        if (n == null) { sb.append("#,"); return; }
        sb.append(n.val).append(",");
        ser(n.left, sb);
        ser(n.right, sb);
    }
    static String serialize(Node root) { StringBuilder sb = new StringBuilder(); ser(root, sb); return sb.toString(); }

    static Node deser(String[] toks, int[] i) {
        if (toks[i[0]].equals("#")) { i[0]++; return null; }
        Node n = new Node(toks[i[0]++]);
        n.left = deser(toks, i);
        n.right = deser(toks, i);
        return n;
    }
    static Node deserialize(String s) {
        String[] toks = s.split(",");
        return deser(toks, new int[]{0});
    }

    public static void main(String[] args) {
        Node node = new Node("root", new Node("left", new Node("left.left"), null), new Node("right"));
        Node back = deserialize(serialize(node));
        System.out.println(back.left.left.val); // left.left
    }
}

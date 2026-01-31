// Day 997: Serialize / deserialize a binary tree.
// Preorder traversal with "#" markers for null children, comma-joined.
// Both serialize and deserialize are O(n) time and space.
import java.util.*;

public class Solution {
    static class Node {
        String val;
        Node left, right;
        Node(String v) { val = v; }
    }

    static void ser(Node n, StringBuilder out) {
        if (n == null) { out.append("#,"); return; }
        out.append(n.val).append(',');
        ser(n.left, out);
        ser(n.right, out);
    }

    static String serialize(Node root) {
        StringBuilder b = new StringBuilder();
        ser(root, b);
        return b.toString();
    }

    static Node deser(Deque<String> toks) {
        String v = toks.pollFirst();
        if (v.equals("#")) return null;
        Node n = new Node(v);
        n.left = deser(toks);
        n.right = deser(toks);
        return n;
    }

    static Node deserialize(String s) {
        Deque<String> toks = new ArrayDeque<>(Arrays.asList(s.split(",")));
        return deser(toks);
    }

    public static void main(String[] args) {
        Node node = new Node("root");
        node.left = new Node("left");
        node.left.left = new Node("left.left");
        node.right = new Node("right");
        String s = serialize(node);
        System.out.println(s);
        assert deserialize(serialize(node)).left.left.val.equals("left.left");
        System.out.println("assertion passed: " + deserialize(s).left.left.val);
    }
}

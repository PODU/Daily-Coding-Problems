// Serialize/deserialize a binary tree via preorder traversal with '#' null markers.
// Time: O(n) for both, Space: O(n).
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {
    static class Node {
        String val;
        Node left, right;
        Node(String v) { val = v; }
        Node(String v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static void ser(Node n, StringBuilder sb) {
        if (n == null) { sb.append("#,"); return; }
        sb.append(n.val).append(','); // assumes values contain no ','
        ser(n.left, sb);
        ser(n.right, sb);
    }
    static String serialize(Node root) { StringBuilder sb = new StringBuilder(); ser(root, sb); return sb.toString(); }

    static Node deser(Deque<String> toks) {
        String t = toks.poll();
        if (t.equals("#")) return null;
        Node n = new Node(t);
        n.left = deser(toks);
        n.right = deser(toks);
        return n;
    }
    static Node deserialize(String s) {
        Deque<String> toks = new ArrayDeque<>();
        for (String p : s.split(",", -1)) toks.add(p);
        return deser(toks);
    }

    public static void main(String[] args) {
        Node node = new Node("root", new Node("left", new Node("left.left"), null), new Node("right"));
        Node d = deserialize(serialize(node));
        System.out.println(d.left.left.val);
    }
}

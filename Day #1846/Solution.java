// Day 1846: Serialize/deserialize a binary tree via preorder traversal with null markers.
// Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static class Node {
        String val; Node left, right;
        Node(String v) { val = v; }
        Node(String v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static void ser(Node n, StringBuilder sb) {
        if (n == null) { sb.append("#,"); return; }
        for (char c : n.val.toCharArray()) {
            if (c == '\\' || c == ',') sb.append('\\');
            sb.append(c);
        }
        sb.append(',');
        ser(n.left, sb);
        ser(n.right, sb);
    }

    static String serialize(Node root) { StringBuilder sb = new StringBuilder(); ser(root, sb); return sb.toString(); }

    static int pos;
    static Node deser(String s) {
        StringBuilder tok = new StringBuilder();
        boolean isNull = s.charAt(pos) == '#';
        while (pos < s.length() && s.charAt(pos) != ',') {
            if (s.charAt(pos) == '\\') { pos++; if (pos < s.length()) tok.append(s.charAt(pos++)); }
            else tok.append(s.charAt(pos++));
        }
        pos++; // skip comma
        if (isNull && tok.toString().equals("#")) return null;
        Node n = new Node(tok.toString());
        n.left = deser(s);
        n.right = deser(s);
        return n;
    }

    static Node deserialize(String s) { pos = 0; return deser(s); }

    public static void main(String[] args) {
        Node node = new Node("root", new Node("left", new Node("left.left"), null), new Node("right"));
        Node round = deserialize(serialize(node));
        System.out.println(round.left.left.val.equals("left.left"));
    }
}

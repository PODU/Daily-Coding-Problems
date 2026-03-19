// Serialize/deserialize a binary tree via preorder with null markers.
// Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static class Node {
        String val;
        Node left, right;
        Node(String v) { val = v; }
        Node(String v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static void ser(Node n, StringBuilder out) {
        if (n == null) { out.append("#|"); return; }
        for (char c : n.val.toCharArray()) {
            if (c == '\\' || c == '|') out.append('\\');
            out.append(c);
        }
        out.append('|');
        ser(n.left, out);
        ser(n.right, out);
    }

    static String serialize(Node root) {
        StringBuilder sb = new StringBuilder();
        ser(root, sb);
        return sb.toString();
    }

    static int[] pos;
    static Node de(List<String> toks) {
        String v = toks.get(pos[0]++);
        if (v.equals("#")) return null;
        Node n = new Node(v);
        n.left = de(toks);
        n.right = de(toks);
        return n;
    }

    static Node deserialize(String s) {
        List<String> toks = new ArrayList<>();
        StringBuilder cur = new StringBuilder();
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == '\\') cur.append(s.charAt(++i));
            else if (c == '|') { toks.add(cur.toString()); cur.setLength(0); }
            else cur.append(c);
        }
        pos = new int[]{0};
        return de(toks);
    }

    public static void main(String[] args) {
        Node node = new Node("root", new Node("left", new Node("left.left"), null), new Node("right"));
        Node d = deserialize(serialize(node));
        System.out.println(d.left.left.val);
    }
}

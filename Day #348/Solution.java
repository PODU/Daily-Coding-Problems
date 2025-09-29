// Ternary Search Tree with insert/search. Each node: char + left/mid/right + end flag.
// Time: O(L * log A) per op, Space: O(total chars).
public class Solution {
    static class Node {
        char c;
        boolean end;
        Node left, mid, right;
        Node(char c) { this.c = c; }
    }
    static Node root;

    static Node insert(Node node, String w, int i) {
        char ch = w.charAt(i);
        if (node == null) node = new Node(ch);
        if (ch < node.c) node.left = insert(node.left, w, i);
        else if (ch > node.c) node.right = insert(node.right, w, i);
        else if (i + 1 < w.length()) node.mid = insert(node.mid, w, i + 1);
        else node.end = true;
        return node;
    }
    static void insert(String w) { if (!w.isEmpty()) root = insert(root, w, 0); }

    static boolean search(String w) {
        Node node = root;
        int i = 0;
        while (node != null) {
            char ch = w.charAt(i);
            if (ch < node.c) node = node.left;
            else if (ch > node.c) node = node.right;
            else {
                if (i + 1 == w.length()) return node.end;
                i++;
                node = node.mid;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        for (String w : new String[]{"code", "cob", "be", "ax", "war", "we"}) insert(w);
        for (String q : new String[]{"code", "cob", "ax", "c", "war", "cat"})
            System.out.println(q + " -> " + search(q));
    }
}

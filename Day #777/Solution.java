// Day 777: Ternary Search Tree with insert and search.
// Each node has left/mid/right children. O(L * log A) per op (L=word length).
public class Solution {
    static class Node {
        char c; boolean end; Node l, m, r;
        Node(char c) { this.c = c; }
    }
    static Node root;

    static Node insert(Node node, String w, int i) {
        char c = w.charAt(i);
        if (node == null) node = new Node(c);
        if (c < node.c) node.l = insert(node.l, w, i);
        else if (c > node.c) node.r = insert(node.r, w, i);
        else if (i + 1 < w.length()) node.m = insert(node.m, w, i + 1);
        else node.end = true;
        return node;
    }
    static void insert(String w) { if (!w.isEmpty()) root = insert(root, w, 0); }

    static boolean search(Node node, String w, int i) {
        if (node == null) return false;
        char c = w.charAt(i);
        if (c < node.c) return search(node.l, w, i);
        if (c > node.c) return search(node.r, w, i);
        if (i + 1 == w.length()) return node.end;
        return search(node.m, w, i + 1);
    }
    static boolean search(String w) { return !w.isEmpty() && search(root, w, 0); }

    public static void main(String[] args) {
        for (String w : new String[]{"code", "cob", "be", "ax", "war", "we"}) insert(w);
        System.out.println(search("cob") + " " + search("code") + " "
                + search("cod") + " " + search("we")); // true true false true
    }
}

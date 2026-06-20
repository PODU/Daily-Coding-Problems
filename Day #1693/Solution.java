// Ternary Search Tree: node has char + left/mid/right + isEnd. Compare char: <left, >right, ==mid & advance.
// Insert/search: O(L * log A) average where L=key length, A=alphabet size.
import java.util.*;

public class Solution {
    static class Node {
        char c;
        boolean isEnd = false;
        Node left, mid, right;
        Node(char c) { this.c = c; }
    }

    static Node insert(Node node, String s, int i) {
        char ch = s.charAt(i);
        if (node == null) node = new Node(ch);
        if (ch < node.c) node.left = insert(node.left, s, i);
        else if (ch > node.c) node.right = insert(node.right, s, i);
        else if (i + 1 < s.length()) node.mid = insert(node.mid, s, i + 1);
        else node.isEnd = true;
        return node;
    }

    static boolean search(Node node, String s, int i) {
        if (node == null) return false;
        char ch = s.charAt(i);
        if (ch < node.c) return search(node.left, s, i);
        if (ch > node.c) return search(node.right, s, i);
        if (i + 1 == s.length()) return node.isEnd;
        return search(node.mid, s, i + 1);
    }

    public static void main(String[] args) {
        Node root = null;
        String[] words = {"code", "cob", "be", "ax", "war", "we"};
        for (String w : words) root = insert(root, w, 0);

        String[] queries = {"code", "cob", "cod", "war", "wa"};
        for (String q : queries)
            System.out.println(search(root, q, 0));
    }
}

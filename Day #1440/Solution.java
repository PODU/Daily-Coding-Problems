// Day 1440: Ternary search tree with insert and search.
// Approach: each node stores a char + left/mid/right; mid advances the word.
// Time: insert/search O(L * log A) avg, Space: O(total chars).
public class Solution {
    static class Node {
        char c;
        boolean isEnd = false;
        Node left, mid, right;
        Node(char ch) { c = ch; }
    }

    static Node insert(Node root, String word, int i) {
        if (i >= word.length()) return root;
        char ch = word.charAt(i);
        if (root == null) root = new Node(ch);
        if (ch < root.c) {
            root.left = insert(root.left, word, i);
        } else if (ch > root.c) {
            root.right = insert(root.right, word, i);
        } else {
            if (i + 1 == word.length()) root.isEnd = true;
            else root.mid = insert(root.mid, word, i + 1);
        }
        return root;
    }

    static boolean search(Node root, String word, int i) {
        if (root == null || i >= word.length()) return false;
        char ch = word.charAt(i);
        if (ch < root.c) return search(root.left, word, i);
        if (ch > root.c) return search(root.right, word, i);
        if (i + 1 == word.length()) return root.isEnd;
        return search(root.mid, word, i + 1);
    }

    public static void main(String[] args) {
        Node root = null;
        for (String w : new String[]{"code", "cob", "be", "ax", "war", "we"})
            root = insert(root, w, 0);
        System.out.println(search(root, "code", 0)); // true
        System.out.println(search(root, "cob", 0));  // true
        System.out.println(search(root, "we", 0));   // true
        System.out.println(search(root, "cod", 0));  // false
        System.out.println(search(root, "cat", 0));  // false
    }
}

// Day 753: Ternary Search Tree with insert and search.
// Insert/Search: O(L + log n) average, O(L * n) worst; L = key length.
public class Solution {
    static class TSTNode {
        char c;
        boolean isEnd = false;
        TSTNode left, mid, right;
        TSTNode(char ch) { c = ch; }
    }

    static class TST {
        TSTNode root;

        TSTNode insert(TSTNode node, String s, int i) {
            char c = s.charAt(i);
            if (node == null) node = new TSTNode(c);
            if (c < node.c)       node.left  = insert(node.left,  s, i);
            else if (c > node.c)  node.right = insert(node.right, s, i);
            else if (i + 1 < s.length())
                node.mid = insert(node.mid, s, i + 1);
            else node.isEnd = true;
            return node;
        }
        void insert(String s) { if (!s.isEmpty()) root = insert(root, s, 0); }

        boolean search(String s) {
            TSTNode node = root;
            int i = 0;
            while (node != null) {
                char c = s.charAt(i);
                if (c < node.c)       node = node.left;
                else if (c > node.c)  node = node.right;
                else {
                    if (i + 1 == s.length()) return node.isEnd;
                    node = node.mid;
                    i++;
                }
            }
            return false;
        }
    }

    public static void main(String[] args) {
        TST tst = new TST();
        for (String w : new String[]{"code", "cob", "be", "ax", "war", "we"})
            tst.insert(w);
        for (String q : new String[]{"code", "cob", "cod", "ax", "hello"})
            System.out.println(q + ": " + tst.search(q));
    }
}

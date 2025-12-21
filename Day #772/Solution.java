// Day 772: Boggle solver. Trie of dictionary + DFS from each cell over 8 neighbors,
// marking visited. O(W) to build trie; search bounded by trie/board size.
import java.util.*;

public class Solution {
    static class Node { Node[] ch = new Node[26]; boolean end; }

    static void insert(Node root, String w) {
        Node cur = root;
        for (char c : w.toCharArray()) {
            int i = c - 'a';
            if (cur.ch[i] == null) cur.ch[i] = new Node();
            cur = cur.ch[i];
        }
        cur.end = true;
    }

    static void dfs(char[][] b, int r, int c, Node node, StringBuilder path, TreeSet<String> out) {
        char ch = b[r][c];
        if (ch == '#') return;
        Node nxt = node.ch[ch - 'a'];
        if (nxt == null) return;
        path.append(ch);
        if (nxt.end) out.add(path.toString());
        b[r][c] = '#';
        for (int dr = -1; dr <= 1; dr++)
            for (int dc = -1; dc <= 1; dc++) {
                if (dr == 0 && dc == 0) continue;
                int nr = r + dr, nc = c + dc;
                if (nr >= 0 && nr < b.length && nc >= 0 && nc < b[0].length)
                    dfs(b, nr, nc, nxt, path, out);
            }
        b[r][c] = ch;
        path.deleteCharAt(path.length() - 1);
    }

    static List<String> solveBoggle(String[] board, String[] dict) {
        Node root = new Node();
        for (String w : dict) insert(root, w);
        char[][] b = new char[board.length][];
        for (int i = 0; i < board.length; i++) b[i] = board[i].toCharArray();
        TreeSet<String> out = new TreeSet<>();
        for (int r = 0; r < b.length; r++)
            for (int c = 0; c < b[0].length; c++)
                dfs(b, r, c, root, new StringBuilder(), out);
        return new ArrayList<>(out);
    }

    public static void main(String[] args) {
        String[] board = {"oaan", "etae", "ihkr", "iflv"};
        String[] dict = {"oath", "pea", "eat", "rain"};
        System.out.println(String.join(" ", solveBoggle(board, dict))); // eat oath
    }
}

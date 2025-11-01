// Boggle solver: build a trie from the dictionary, DFS each cell over 8 neighbors
// (each cell used once per path), collect words present in the trie.
// Time: O(cells * 8^L) bounded by trie depth; Space: O(total dictionary chars).
import java.util.*;

public class Solution {
    static class Trie {
        Trie[] next = new Trie[26];
        boolean word = false;
    }

    static char[][] board;
    static int R, C;
    static TreeSet<String> found = new TreeSet<>();

    static void dfs(int r, int c, Trie node, StringBuilder cur, boolean[][] used) {
        if (r < 0 || r >= R || c < 0 || c >= C || used[r][c]) return;
        char ch = board[r][c];
        Trie nxt = node.next[ch - 'a'];
        if (nxt == null) return;
        cur.append(ch);
        used[r][c] = true;
        if (nxt.word) found.add(cur.toString());
        for (int dr = -1; dr <= 1; dr++)
            for (int dc = -1; dc <= 1; dc++)
                if (dr != 0 || dc != 0) dfs(r + dr, c + dc, nxt, cur, used);
        used[r][c] = false;
        cur.deleteCharAt(cur.length() - 1);
    }

    public static void main(String[] args) {
        board = new char[][]{
            {'o','a','a','n'},{'e','t','a','e'},{'i','h','k','r'},{'i','f','l','v'}};
        String[] dict = {"oath","pea","eat","rain"};
        R = board.length; C = board[0].length;

        Trie root = new Trie();
        for (String w : dict) {
            Trie node = root;
            for (char ch : w.toCharArray()) {
                if (node.next[ch - 'a'] == null) node.next[ch - 'a'] = new Trie();
                node = node.next[ch - 'a'];
            }
            node.word = true;
        }

        boolean[][] used = new boolean[R][C];
        StringBuilder cur = new StringBuilder();
        for (int r = 0; r < R; r++)
            for (int c = 0; c < C; c++)
                dfs(r, c, root, cur, used);

        System.out.println("[" + String.join(", ", found) + "]");
    }
}

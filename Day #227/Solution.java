// Boggle solver: build a Trie of the dictionary, DFS from every cell over 8 neighbours.
// Time: O(cells * 8^L) bounded by Trie; Space: O(dictionary size).
import java.util.*;

public class Solution {
    static class Trie {
        Trie[] next = new Trie[26];
        boolean word = false;
    }

    static void dfs(char[][] b, int r, int c, Trie node, StringBuilder cur, TreeSet<String> found) {
        if (r < 0 || c < 0 || r >= b.length || c >= b[0].length || b[r][c] == '#') return;
        char ch = b[r][c];
        Trie nx = node.next[ch - 'a'];
        if (nx == null) return;
        cur.append(ch);
        if (nx.word) found.add(cur.toString());
        b[r][c] = '#';
        for (int dr = -1; dr <= 1; dr++)
            for (int dc = -1; dc <= 1; dc++)
                if (dr != 0 || dc != 0) dfs(b, r + dr, c + dc, nx, cur, found);
        b[r][c] = ch;
        cur.deleteCharAt(cur.length() - 1);
    }

    static List<String> boggle(char[][] board, String[] dict) {
        Trie root = new Trie();
        for (String w : dict) {
            Trie n = root;
            for (char ch : w.toCharArray()) {
                if (n.next[ch - 'a'] == null) n.next[ch - 'a'] = new Trie();
                n = n.next[ch - 'a'];
            }
            n.word = true;
        }
        TreeSet<String> found = new TreeSet<>();
        StringBuilder cur = new StringBuilder();
        for (int r = 0; r < board.length; r++)
            for (int c = 0; c < board[0].length; c++)
                dfs(board, r, c, root, cur, found);
        return new ArrayList<>(found);
    }

    public static void main(String[] args) {
        char[][] board = {
            {'o', 'a', 'a', 'n'},
            {'e', 't', 'a', 'e'},
            {'i', 'h', 'k', 'r'},
            {'i', 'f', 'l', 'v'}};
        String[] dict = {"oath", "pea", "eat", "rain"};
        List<String> res = boggle(board, dict);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("'").append(res.get(i)).append("'");
            if (i + 1 < res.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}

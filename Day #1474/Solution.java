// Day 1474: Boggle solver. Trie of dictionary + DFS from each cell over
// 8-adjacent neighbors (no reuse), pruned by trie prefixes.
// Time O(R*C*8^L) worst case; Space O(total dict chars).
import java.util.*;

public class Solution {
    static class Node {
        Map<Character, Node> next = new HashMap<>();
        String word;
    }

    static void dfs(char[][] b, int r, int c, Node node, TreeSet<String> found) {
        char ch = b[r][c];
        if (ch == '*' || !node.next.containsKey(ch)) return;
        Node nxt = node.next.get(ch);
        if (nxt.word != null) found.add(nxt.word);
        b[r][c] = '*';
        for (int dr = -1; dr <= 1; ++dr)
            for (int dc = -1; dc <= 1; ++dc) {
                if (dr == 0 && dc == 0) continue;
                int nr = r + dr, nc = c + dc;
                if (nr >= 0 && nr < b.length && nc >= 0 && nc < b[0].length)
                    dfs(b, nr, nc, nxt, found);
            }
        b[r][c] = ch;
    }

    public static void main(String[] args) {
        char[][] board = {
            {'o', 'a', 'a', 'n'},
            {'e', 't', 'a', 'e'},
            {'i', 'h', 'k', 'r'},
            {'i', 'f', 'l', 'v'},
        };
        String[] words = {"oath", "pea", "eat", "rain"};

        Node root = new Node();
        for (String w : words) {
            Node node = root;
            for (char ch : w.toCharArray())
                node = node.next.computeIfAbsent(ch, k -> new Node());
            node.word = w;
        }

        TreeSet<String> found = new TreeSet<>();
        for (int r = 0; r < board.length; ++r)
            for (int c = 0; c < board[0].length; ++c)
                dfs(board, r, c, root, found);

        System.out.println(found);  // [eat, oath]
    }
}

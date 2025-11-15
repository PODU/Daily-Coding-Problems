// Day 599: Ghost game - find starting letters that guarantee player 1 a win.
// Approach: build a trie, minimax over it (landing on a word loses). Time O(total chars), Space O(trie).
import java.util.*;

public class Solution {
    static class Trie {
        TreeMap<Character, Trie> ch = new TreeMap<>();
        boolean isWord = false;
    }

    static void insert(Trie root, String w) {
        Trie node = root;
        for (char c : w.toCharArray())
            node = node.ch.computeIfAbsent(c, k -> new Trie());
        node.isWord = true;
    }

    // True if the player to move from `node` can force the opponent to lose.
    static boolean moverWins(Trie node) {
        for (Trie child : node.ch.values()) {
            if (child.isWord) continue;
            if (!moverWins(child)) return true;
        }
        return false;
    }

    public static void main(String[] args) {
        String[] dict = {"cat", "calf", "dog", "bear"};
        Trie root = new Trie();
        for (String w : dict) insert(root, w);

        List<Character> winning = new ArrayList<>();
        for (Map.Entry<Character, Trie> e : root.ch.entrySet())
            if (!e.getValue().isWord && !moverWins(e.getValue()))
                winning.add(e.getKey());

        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < winning.size(); i++) {
            if (i > 0) sb.append(' ');
            sb.append(winning.get(i));
        }
        System.out.println(sb.toString());
    }
}

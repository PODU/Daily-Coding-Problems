// Day 1122 - Ghost: winning starting letters for player 1 under optimal play
// Trie + minimax over prefixes. A mover loses if every letter completes a word
// or leads to a winning position for the opponent. Time: O(total letters).
import java.util.*;

public class Solution {
    static class TrieNode {
        Map<Character, TrieNode> children = new TreeMap<>();
        boolean isWord = false;
    }

    static void insert(TrieNode root, String w) {
        TrieNode node = root;
        for (char ch : w.toCharArray())
            node = node.children.computeIfAbsent(ch, k -> new TrieNode());
        node.isWord = true;
    }

    static boolean canWin(TrieNode node) {
        for (TrieNode child : node.children.values()) {
            if (child.isWord) continue;
            if (!canWin(child)) return true;
        }
        return false;
    }

    public static void main(String[] args) {
        String[] words = {"cat", "calf", "dog", "bear"};
        TrieNode root = new TrieNode();
        for (String w : words) insert(root, w);

        List<Character> res = new ArrayList<>();
        for (Map.Entry<Character, TrieNode> e : root.children.entrySet())
            if (!e.getValue().isWord && !canWin(e.getValue())) res.add(e.getKey());
        Collections.sort(res);

        StringBuilder sb = new StringBuilder("Winning starting letters:");
        for (char c : res) sb.append(" ").append(c);
        System.out.println(sb);
    }
}

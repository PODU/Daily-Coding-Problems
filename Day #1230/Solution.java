// Trie with pass-through counts; for each word walk down until count==1 to get its shortest unique prefix.
// Time: O(total chars), Space: O(total chars).
import java.util.*;

public class Solution {
    static class Trie {
        Map<Character, Trie> next = new HashMap<>();
        int count = 0;
    }

    static void insert(Trie root, String w) {
        Trie cur = root;
        for (char c : w.toCharArray()) {
            cur = cur.next.computeIfAbsent(c, k -> new Trie());
            cur.count++;
        }
    }

    static String prefix(Trie root, String w) {
        Trie cur = root;
        StringBuilder p = new StringBuilder();
        for (char c : w.toCharArray()) {
            cur = cur.next.get(c);
            p.append(c);
            if (cur.count == 1) break;
        }
        return p.toString();
    }

    public static void main(String[] args) {
        String[] words = {"dog", "cat", "apple", "apricot", "fish"};
        Trie root = new Trie();
        for (String w : words) insert(root, w);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < words.length; i++) {
            sb.append(prefix(root, words[i]));
            if (i + 1 < words.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}

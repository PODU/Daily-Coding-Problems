// Day 949: Autocomplete - return all words having query as a prefix, using a Trie.
// Build O(total chars); query O(|s| + matches). Insertion order preserved.
import java.util.*;

public class Solution {
    static class Node {
        Map<Character, Node> next = new HashMap<>();
        List<Integer> ids = new ArrayList<>();
    }

    static class Trie {
        Node root = new Node();
        void insert(String w, int id) {
            Node cur = root;
            for (char c : w.toCharArray()) {
                cur = cur.next.computeIfAbsent(c, k -> new Node());
                cur.ids.add(id);
            }
        }
        List<Integer> withPrefix(String s) {
            Node cur = root;
            for (char c : s.toCharArray()) {
                cur = cur.next.get(c);
                if (cur == null) return Collections.emptyList();
            }
            return cur.ids;
        }
    }

    public static void main(String[] args) {
        String[] words = {"dog", "deer", "deal"};
        Trie t = new Trie();
        for (int i = 0; i < words.length; i++) t.insert(words[i], i);
        List<Integer> res = t.withPrefix("de");
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append(words[res.get(i)]);
        }
        sb.append("]");
        System.out.println(sb); // [deer, deal]
    }
}

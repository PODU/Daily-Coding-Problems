// Day 1475: Autocomplete via trie. Walk to prefix node, collect subtree words.
// Build O(total chars); query O(len(prefix) + matches). Space O(total chars).
import java.util.*;

public class Solution {
    static class Node {
        TreeMap<Character, Node> next = new TreeMap<>();
        int order = -1;
        String word;
    }

    static class Trie {
        Node root = new Node();

        void insert(String w, int order) {
            Node node = root;
            for (char ch : w.toCharArray())
                node = node.next.computeIfAbsent(ch, k -> new Node());
            node.order = order;
            node.word = w;
        }

        void collect(Node n, List<int[]> ordIdx, List<String> words) {
            if (n.order >= 0) { ordIdx.add(new int[]{n.order, words.size()}); words.add(n.word); }
            for (Node child : n.next.values()) collect(child, ordIdx, words);
        }

        List<String> startsWith(String prefix) {
            Node node = root;
            for (char ch : prefix.toCharArray()) {
                node = node.next.get(ch);
                if (node == null) return new ArrayList<>();
            }
            List<int[]> ordIdx = new ArrayList<>();
            List<String> words = new ArrayList<>();
            collect(node, ordIdx, words);
            ordIdx.sort((a, b) -> Integer.compare(a[0], b[0]));
            List<String> res = new ArrayList<>();
            for (int[] oi : ordIdx) res.add(words.get(oi[1]));
            return res;
        }
    }

    public static void main(String[] args) {
        Trie t = new Trie();
        String[] dict = {"dog", "deer", "deal"};
        for (int i = 0; i < dict.length; ++i) t.insert(dict[i], i);
        System.out.println(t.startsWith("de"));  // [deer, deal]
    }
}

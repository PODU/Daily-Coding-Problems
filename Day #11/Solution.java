// Autocomplete via Trie: insert all words, walk to prefix node, DFS collect.
// Build: O(total chars); query: O(|prefix| + matches). Results in insertion order.
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

public class Solution {
    static class TrieNode {
        Map<Character, TrieNode> ch = new TreeMap<>();
        int order = -1;
    }

    static class Trie {
        TrieNode root = new TrieNode();
        int counter = 0;

        void insert(String w) {
            TrieNode cur = root;
            for (char c : w.toCharArray())
                cur = cur.ch.computeIfAbsent(c, k -> new TrieNode());
            cur.order = counter++;
        }

        List<String> autocomplete(String prefix) {
            TrieNode cur = root;
            for (char c : prefix.toCharArray()) {
                cur = cur.ch.get(c);
                if (cur == null) return new ArrayList<>();
            }
            List<int[]> idx = new ArrayList<>();
            List<String> words = new ArrayList<>();
            dfs(cur, new StringBuilder(prefix), idx, words);
            // order by insertion index
            List<String> res = new ArrayList<>();
            idx.sort((a, b) -> Integer.compare(a[0], b[0]));
            for (int[] p : idx) res.add(words.get(p[1]));
            return res;
        }

        void dfs(TrieNode n, StringBuilder buf, List<int[]> idx, List<String> words) {
            if (n.order >= 0) { idx.add(new int[]{n.order, words.size()}); words.add(buf.toString()); }
            for (Map.Entry<Character, TrieNode> e : n.ch.entrySet()) {
                buf.append(e.getKey());
                dfs(e.getValue(), buf, idx, words);
                buf.deleteCharAt(buf.length() - 1);
            }
        }
    }

    public static void main(String[] args) {
        Trie t = new Trie();
        for (String w : new String[]{"dog", "deer", "deal"}) t.insert(w);
        System.out.println(t.autocomplete("de"));
    }
}

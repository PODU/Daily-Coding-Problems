// Day 692: Autocomplete - return all dictionary strings having s as a prefix.
// Approach: Trie. Insert words O(total chars); query walks prefix then DFS to
// collect words. Query O(|s| + #matches * len).
import java.util.*;

public class Solution {
    static class Node { Map<Character, Node> ch = new HashMap<>(); boolean end; }
    static Node root = new Node();

    static void insert(String w) {
        Node cur = root;
        for (char c : w.toCharArray()) cur = cur.ch.computeIfAbsent(c, k -> new Node());
        cur.end = true;
    }

    static void dfs(Node n, StringBuilder cur, List<String> out) {
        if (n.end) out.add(cur.toString());
        for (Map.Entry<Character, Node> e : n.ch.entrySet()) {
            cur.append(e.getKey()); dfs(e.getValue(), cur, out); cur.deleteCharAt(cur.length() - 1);
        }
    }

    static List<String> autocomplete(String s) {
        Node cur = root;
        for (char c : s.toCharArray()) { cur = cur.ch.get(c); if (cur == null) return new ArrayList<>(); }
        List<String> out = new ArrayList<>();
        dfs(cur, new StringBuilder(s), out);
        return out;
    }

    public static void main(String[] args) {
        for (String w : new String[]{"dog", "deer", "deal"}) insert(w);
        List<String> res = autocomplete("de");
        Collections.sort(res);
        System.out.println(res);   // [deal, deer]
    }
}

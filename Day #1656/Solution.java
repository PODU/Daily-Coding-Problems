// Trie autocomplete: insert words, DFS from prefix node in child-insertion order
// (LinkedHashMap). O(total chars) build, O(matches) query; O(total chars) space.
import java.util.*;

public class Solution {
    static class Node {
        LinkedHashMap<Character, Node> children = new LinkedHashMap<>();
        String word = null;
    }

    static Node root = new Node();

    static void insert(String w) {
        Node n = root;
        for (char c : w.toCharArray()) {
            n.children.putIfAbsent(c, new Node());
            n = n.children.get(c);
        }
        n.word = w;
    }

    static void dfs(Node n, List<String> out) {
        if (n.word != null) out.add(n.word);
        for (Node c : n.children.values()) dfs(c, out);
    }

    static List<String> autocomplete(String s) {
        Node n = root;
        for (char c : s.toCharArray()) {
            n = n.children.get(c);
            if (n == null) return new ArrayList<>();
        }
        List<String> out = new ArrayList<>();
        dfs(n, out);
        return out;
    }

    public static void main(String[] a) {
        for (String w : new String[]{"dog", "deer", "deal"}) insert(w);
        List<String> res = autocomplete("de");
        System.out.println("[" + String.join(", ", res) + "]");
    }
}

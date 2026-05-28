// Shortest unique prefix via Trie storing char frequency counts. O(total chars) time & space.
import java.util.*;

public class Solution {
    static class Node { Map<Character, Node> ch = new HashMap<>(); int cnt = 0; }

    public static void main(String[] args) {
        String[] words = {"dog", "cat", "apple", "apricot", "fish"};
        Node root = new Node();
        for (String w : words) {
            Node cur = root;
            for (char c : w.toCharArray()) {
                cur.ch.putIfAbsent(c, new Node());
                cur = cur.ch.get(c);
                cur.cnt++;
            }
        }
        StringBuilder out = new StringBuilder();
        for (String w : words) {
            Node cur = root;
            StringBuilder pre = new StringBuilder();
            for (char c : w.toCharArray()) {
                cur = cur.ch.get(c);
                pre.append(c);
                if (cur.cnt == 1) break;
            }
            out.append(pre).append("\n");
        }
        System.out.print(out.toString());
    }
}

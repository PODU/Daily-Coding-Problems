// Day 1676: PrefixMapSum via trie storing cumulative values + delta on overwrite.
// insert/sum both O(key length). Space O(total chars).
import java.util.*;

public class Solution {
    static class Node { long total = 0; Map<Character, Node> next = new HashMap<>(); }

    static class PrefixMapSum {
        Node root = new Node();
        Map<String, Long> vals = new HashMap<>();

        void insert(String key, long value) {
            long delta = value - vals.getOrDefault(key, 0L);
            vals.put(key, value);
            Node cur = root;
            for (char c : key.toCharArray()) {
                cur = cur.next.computeIfAbsent(c, k -> new Node());
                cur.total += delta;
            }
        }
        long sum(String prefix) {
            Node cur = root;
            for (char c : prefix.toCharArray()) {
                cur = cur.next.get(c);
                if (cur == null) return 0;
            }
            return cur.total;
        }
    }

    public static void main(String[] args) {
        PrefixMapSum m = new PrefixMapSum();
        m.insert("columnar", 3);
        System.out.println(m.sum("col")); // 3
        m.insert("column", 2);
        System.out.println(m.sum("col")); // 5
    }
}

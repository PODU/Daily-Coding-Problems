// Day 799: PrefixMapSum - trie where each node stores sum of values below it.
// insert overwrites via delta (new-old) propagated along the path.
// insert O(L), sum O(L). Space O(total chars).
import java.util.*;

public class Solution {
    static class Node {
        long total = 0;
        Node[] child = new Node[26];
    }

    static class PrefixMapSum {
        Node root = new Node();
        Map<String, Integer> vals = new HashMap<>();

        void insert(String key, int value) {
            int delta = value - vals.getOrDefault(key, 0);
            vals.put(key, value);
            Node cur = root;
            cur.total += delta;
            for (char c : key.toCharArray()) {
                int i = c - 'a';
                if (cur.child[i] == null) cur.child[i] = new Node();
                cur = cur.child[i];
                cur.total += delta;
            }
        }

        long sum(String prefix) {
            Node cur = root;
            for (char c : prefix.toCharArray()) {
                int i = c - 'a';
                if (cur.child[i] == null) return 0;
                cur = cur.child[i];
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

// PrefixMapSum: Trie where each node stores the running sum of values passing through it.
// On overwrite, propagate delta = new - old. insert/sum both O(key length).
import java.util.*;

public class Solution {
    static class Node {
        Map<Character, Node> next = new HashMap<>();
        int sum = 0;
    }

    static class PrefixMapSum {
        Node root = new Node();
        Map<String, Integer> vals = new HashMap<>();

        void insert(String key, int value) {
            int delta = value - vals.getOrDefault(key, 0);
            vals.put(key, value);
            Node n = root;
            for (char c : key.toCharArray()) {
                n = n.next.computeIfAbsent(c, k -> new Node());
                n.sum += delta;
            }
        }

        int sum(String prefix) {
            Node n = root;
            for (char c : prefix.toCharArray()) {
                n = n.next.get(c);
                if (n == null) return 0;
            }
            return n.sum;
        }
    }

    public static void main(String[] args) {
        PrefixMapSum mapsum = new PrefixMapSum();
        mapsum.insert("columnar", 3);
        System.out.println(mapsum.sum("col")); // 3
        mapsum.insert("column", 2);
        System.out.println(mapsum.sum("col")); // 5
    }
}

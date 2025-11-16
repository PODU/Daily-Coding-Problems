// Day 613: PrefixMapSum - insert(key,value) and sum(prefix).
// Approach: trie where each node stores total of values passing through; insert propagates delta. Time O(|key|).
import java.util.*;

public class Solution {
    static class Node {
        long sum = 0;
        Map<Character, Node> ch = new HashMap<>();
    }

    static class PrefixMapSum {
        Node root = new Node();
        Map<String, Long> values = new HashMap<>();

        void insert(String key, long value) {
            long delta = value - values.getOrDefault(key, 0L);
            values.put(key, value);
            Node node = root;
            for (char c : key.toCharArray()) {
                node = node.ch.computeIfAbsent(c, k -> new Node());
                node.sum += delta;
            }
        }

        long sum(String prefix) {
            Node node = root;
            for (char c : prefix.toCharArray()) {
                node = node.ch.get(c);
                if (node == null) return 0;
            }
            return node.sum;
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

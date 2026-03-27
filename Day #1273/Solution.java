// Day 1273: PrefixMapSum - insert(key,value) and sum(prefix).
// Trie storing accumulated sums; insert applies the delta vs the old value.
// insert/sum are O(key length).
import java.util.*;

public class Solution {
    static class PrefixMapSum {
        static class TrieNode { long sum = 0; Map<Character, TrieNode> next = new HashMap<>(); }
        TrieNode root = new TrieNode();
        Map<String, Long> vals = new HashMap<>();

        void insert(String key, long value) {
            long delta = value - vals.getOrDefault(key, 0L);
            vals.put(key, value);
            TrieNode node = root;
            for (char c : key.toCharArray()) {
                node = node.next.computeIfAbsent(c, k -> new TrieNode());
                node.sum += delta;
            }
        }

        long sum(String prefix) {
            TrieNode node = root;
            for (char c : prefix.toCharArray()) {
                node = node.next.get(c);
                if (node == null) return 0;
            }
            return node.sum;
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

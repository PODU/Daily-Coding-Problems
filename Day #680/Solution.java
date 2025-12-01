// Trie with per-node pass counts; shortest unique prefix = up to first node count==1. Time O(total chars).
import java.util.*;

public class Solution {
    static class Node {
        Map<Character, Node> next = new HashMap<>();
        int count = 0;
    }

    static List<String> shortestUniquePrefixes(List<String> words) {
        Node root = new Node();
        for (String w : words) {
            Node cur = root;
            for (char c : w.toCharArray()) {
                cur = cur.next.computeIfAbsent(c, k -> new Node());
                cur.count++;
            }
        }
        List<String> res = new ArrayList<>();
        for (String w : words) {
            Node cur = root;
            StringBuilder pre = new StringBuilder();
            for (char c : w.toCharArray()) {
                cur = cur.next.get(c);
                pre.append(c);
                if (cur.count == 1) break;
            }
            res.add(pre.toString());
        }
        return res;
    }

    public static void main(String[] args) {
        List<String> words = Arrays.asList("dog", "cat", "apple", "apricot", "fish");
        System.out.println(shortestUniquePrefixes(words));
    }
}

// Day 162: Shortest unique prefix via trie. Each node stores pass-through count;
// the shortest prefix with count 1 is unique. Time O(total chars), Space O(same).
import java.util.*;

public class Solution {
    static class Node {
        int count = 0;
        Map<Character, Node> child = new HashMap<>();
    }

    public static void main(String[] args) {
        String[] words = {"dog", "cat", "apple", "apricot", "fish"};
        Node root = new Node();
        for (String w : words) {
            Node cur = root;
            for (char c : w.toCharArray()) {
                cur = cur.child.computeIfAbsent(c, k -> new Node());
                cur.count++;
            }
        }
        StringBuilder out = new StringBuilder();
        for (String w : words) {
            Node cur = root;
            StringBuilder prefix = new StringBuilder();
            for (char c : w.toCharArray()) {
                cur = cur.child.get(c);
                prefix.append(c);
                if (cur.count == 1) break;
            }
            out.append(prefix).append('\n');
        }
        System.out.print(out);
    }
}

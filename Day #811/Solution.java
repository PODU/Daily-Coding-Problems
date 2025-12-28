// Shortest unique prefix via trie of char frequency counts.
// Walk each word until count==1. Time O(total chars), Space O(total chars).
import java.util.*;

public class Solution {
    static class Node {
        int cnt = 0;
        Map<Character, Node> next = new HashMap<>();
    }

    static void insert(Node root, String w) {
        Node cur = root;
        for (char c : w.toCharArray()) {
            cur.next.putIfAbsent(c, new Node());
            cur = cur.next.get(c);
            cur.cnt++;
        }
    }

    static String shortestPrefix(Node root, String w) {
        Node cur = root;
        StringBuilder pre = new StringBuilder();
        for (char c : w.toCharArray()) {
            cur = cur.next.get(c);
            pre.append(c);
            if (cur.cnt == 1) break;
        }
        return pre.toString();
    }

    public static void main(String[] args) {
        String[] words = {"dog", "cat", "apple", "apricot", "fish"};
        Node root = new Node();
        for (String w : words) insert(root, w);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < words.length; i++) {
            if (i > 0) sb.append(", ");
            sb.append(shortestPrefix(root, words[i]));
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}

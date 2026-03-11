// Ghost word game: trie + game theory. canWin(node)=mover can force win.
// Winning start c: child not a word AND opponent (canWin(child)) cannot win.
// Time O(total chars), Space O(total chars).
// NOTE: README sample shows only "b" but "c" is also winning.
import java.util.*;

public class Solution {
    static class Node {
        TreeMap<Character, Node> ch = new TreeMap<>();
        boolean isWord = false;
    }

    static void insert(Node root, String w) {
        Node cur = root;
        for (char c : w.toCharArray()) {
            cur.ch.putIfAbsent(c, new Node());
            cur = cur.ch.get(c);
        }
        cur.isWord = true;
    }

    // can the player about to move from this prefix force a win?
    static boolean canWin(Node node) {
        for (Node child : node.ch.values()) {
            if (child.isWord) continue;        // completing a word loses
            if (!canWin(child)) return true;   // opponent loses
        }
        return false;
    }

    public static void main(String[] args) {
        String[] dict = {"cat", "calf", "dog", "bear"};
        Node root = new Node();
        for (String w : dict) insert(root, w);

        List<Character> wins = new ArrayList<>();
        for (Map.Entry<Character, Node> e : root.ch.entrySet()) {
            Node child = e.getValue();
            if (!child.isWord && !canWin(child)) wins.add(e.getKey());
        }
        Collections.sort(wins);

        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < wins.size(); i++) {
            if (i > 0) sb.append(' ');
            sb.append(wins.get(i));
        }
        System.out.println(sb.toString());
    }
}

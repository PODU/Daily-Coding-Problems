// Day 259: Ghost word game. Build a trie of the dictionary. A starting letter is
// guaranteed safe for player 1 iff every word in its subtree has even length, so
// the opponent is always the one forced to complete a word. Trie DFS, O(total chars).
import java.util.*;

public class Solution {
    static class T { Map<Character, T> kids = new TreeMap<>(); boolean word = false; }

    static void insert(T root, String w) {
        T n = root;
        for (char c : w.toCharArray()) { n.kids.putIfAbsent(c, new T()); n = n.kids.get(c); }
        n.word = true;
    }

    static boolean allEven(T n, int depth) {
        if (n.word && depth % 2 != 0) return false;
        for (T ch : n.kids.values()) if (!allEven(ch, depth + 1)) return false;
        return true;
    }

    public static void main(String[] args) {
        String[] words = {"cat", "calf", "dog", "bear"};
        T root = new T();
        for (String w : words) insert(root, w);

        StringBuilder sb = new StringBuilder(); // TreeMap -> sorted order
        for (Map.Entry<Character, T> e : root.kids.entrySet())
            if (allEven(e.getValue(), 1)) sb.append(e.getKey());
        System.out.println(sb);
    }
}

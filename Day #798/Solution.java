// Day 798: Sentence equivalence via synonym pairs (NOT transitive).
// Store each unordered pair in a set; words match if equal or directly paired.
// Time O(W) per comparison, Space O(P).
import java.util.*;

public class Solution {
    static Set<String> syn = new HashSet<>();

    static String key(String a, String b) {
        return a.compareTo(b) <= 0 ? a + "\0" + b : b + "\0" + a;
    }

    static void add(String a, String b) { syn.add(key(a, b)); }

    static boolean same(String a, String b) {
        return a.equals(b) || syn.contains(key(a, b));
    }

    static boolean equivalent(String[] s1, String[] s2) {
        if (s1.length != s2.length) return false;
        for (int i = 0; i < s1.length; i++)
            if (!same(s1[i], s2[i])) return false;
        return true;
    }

    public static void main(String[] args) {
        add("big", "large");
        add("eat", "consume");
        String[] a = {"He", "wants", "to", "eat", "food."};
        String[] b = {"He", "wants", "to", "consume", "food."};
        System.out.println(equivalent(a, b) ? "True (equivalent)" : "False");
    }
}

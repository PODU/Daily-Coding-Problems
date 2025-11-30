// Day 675: Sentence equivalence under (non-transitive) synonym pairs. For each position,
// words must be equal or a known synonym pair. Time O(W) with a hashed pair set.
import java.util.*;

public class Solution {
    static String[] tokens(String s) {
        String[] raw = s.toLowerCase().split("[^a-z0-9]+");
        List<String> out = new ArrayList<>();
        for (String w : raw) if (!w.isEmpty()) out.add(w);
        return out.toArray(new String[0]);
    }

    static boolean equivalent(String[][] syn, String s1, String s2) {
        Set<String> S = new HashSet<>();
        for (String[] p : syn) { S.add(p[0] + "\0" + p[1]); S.add(p[1] + "\0" + p[0]); }
        String[] a = tokens(s1), b = tokens(s2);
        if (a.length != b.length) return false;
        for (int i = 0; i < a.length; i++)
            if (!a[i].equals(b[i]) && !S.contains(a[i] + "\0" + b[i])) return false;
        return true;
    }

    public static void main(String[] args) {
        String[][] syn = {{"big", "large"}, {"eat", "consume"}};
        System.out.println(equivalent(syn, "He wants to eat food.", "He wants to consume food.")
            ? "True" : "False"); // True
    }
}

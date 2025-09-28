// Sentence Similarity. Direct (non-transitive) synonym pairs via hash set.
// Time O(P + N) for P pairs and N words. Space O(P).
// Secondary union-find approach (transitive follow-up) commented below.
import java.util.*;

public class Solution {
    static String[] tokenize(String s) {
        List<String> out = new ArrayList<>();
        for (String w : s.trim().split("\\s+")) {
            while (!w.isEmpty() && !Character.isLetterOrDigit(w.charAt(w.length() - 1)))
                w = w.substring(0, w.length() - 1);
            out.add(w);
        }
        return out.toArray(new String[0]);
    }

    static boolean areSimilar(String[][] synonyms, String s1, String s2) {
        Set<String> pairs = new HashSet<>();
        for (String[] p : synonyms) {
            pairs.add(p[0] + "\0" + p[1]);
            pairs.add(p[1] + "\0" + p[0]);
        }
        String[] w1 = tokenize(s1), w2 = tokenize(s2);
        if (w1.length != w2.length) return false;
        for (int i = 0; i < w1.length; i++) {
            if (w1[i].equals(w2[i])) continue;
            if (pairs.contains(w1[i] + "\0" + w2[i])) continue;
            return false;
        }
        return true;
    }

    // --- Follow-up (transitive): union-find ---
    // Build DSU over all synonym tokens; union each pair. Two words match at a
    // position if equal OR find(w1) == find(w2). Same O(P*alpha + N) overall.

    public static void main(String[] args) {
        String[][] synonyms = {{"big", "large"}, {"eat", "consume"}};
        String s1 = "He wants to eat food.";
        String s2 = "He wants to consume food.";
        System.out.println(areSimilar(synonyms, s1, s2) ? "equivalent" : "not equivalent");
    }
}

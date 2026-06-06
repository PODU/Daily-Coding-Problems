// Day 1623: Sentence equivalence via synonym set.
// Build symmetric synonym set; compare word-by-word. Time O(W), Space O(S).
import java.util.*;

public class Solution {
    static boolean equivalent(String a, String b, String[][] syns) {
        Set<String> pairs = new HashSet<>();
        for (String[] p : syns) {
            pairs.add(p[0] + "\0" + p[1]);
            pairs.add(p[1] + "\0" + p[0]);
        }
        String[] wa = a.split("\\s+"), wb = b.split("\\s+");
        if (wa.length != wb.length) return false;
        for (int i = 0; i < wa.length; i++)
            if (!wa[i].equals(wb[i]) && !pairs.contains(wa[i] + "\0" + wb[i])) return false;
        return true;
    }

    public static void main(String[] args) {
        String[][] syns = {{"big", "large"}, {"eat", "consume"}};
        boolean eq = equivalent("He wants to eat food.", "He wants to consume food.", syns);
        System.out.println(eq ? "The two sentences are equivalent." : "The two sentences are not equivalent.");
    }
}

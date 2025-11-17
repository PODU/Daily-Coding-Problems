// Cryptarithmetic solver via backtracking over distinct letters with column-sum check.
// Time: O(10!) worst case over distinct letters (<=10), Space: O(#letters).
import java.util.*;

public class Solution {
    static String w1, w2, w3;
    static char[] letters;
    static Set<Character> leading = new HashSet<>();
    static Map<Character,Integer> assign = new HashMap<>();
    static boolean[] used = new boolean[10];

    static long valueOf(String w) {
        long v = 0;
        for (char c : w.toCharArray()) v = v * 10 + assign.get(c);
        return v;
    }

    static boolean solve(int idx) {
        if (idx == letters.length)
            return valueOf(w1) + valueOf(w2) == valueOf(w3);
        char ch = letters[idx];
        for (int d = 0; d <= 9; d++) {
            if (used[d]) continue;
            if (d == 0 && leading.contains(ch)) continue;
            used[d] = true; assign.put(ch, d);
            if (solve(idx + 1)) return true;
            used[d] = false;
        }
        return false;
    }

    public static void main(String[] args) {
        w1 = "SEND"; w2 = "MORE"; w3 = "MONEY";
        leading.add(w1.charAt(0)); leading.add(w2.charAt(0)); leading.add(w3.charAt(0));
        LinkedHashSet<Character> uniq = new LinkedHashSet<>();
        for (char c : (w1 + w2 + w3).toCharArray()) uniq.add(c);
        letters = new char[uniq.size()];
        int i = 0; for (char c : uniq) letters[i++] = c;

        if (solve(0)) {
            StringBuilder sb = new StringBuilder("{");
            boolean first = true;
            for (char c : uniq) {
                if (!first) sb.append(", ");
                first = false;
                sb.append("'").append(c).append("': ").append(assign.get(c));
            }
            sb.append("}");
            System.out.println(sb);
        } else {
            System.out.println("No solution");
        }
    }
}

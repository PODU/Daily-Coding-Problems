// Cryptarithmetic solver via backtracking over distinct letters. O(10!/(10-k)!) worst, pruned.
import java.util.*;

public class Solution {
    static String W1 = "SEND", W2 = "MORE", W3 = "MONEY";
    static List<Character> letters = new ArrayList<>();
    static Map<Character, Integer> assign = new HashMap<>();
    static boolean[] used = new boolean[10];
    static Set<Character> leading = new HashSet<>();

    static long val(String w) {
        long v = 0;
        for (char c : w.toCharArray()) v = v * 10 + assign.get(c);
        return v;
    }

    static boolean solve(int idx) {
        if (idx == letters.size()) return val(W1) + val(W2) == val(W3);
        char c = letters.get(idx);
        for (int d = 0; d <= 9; d++) {
            if (used[d]) continue;
            if (d == 0 && leading.contains(c)) continue;
            used[d] = true; assign.put(c, d);
            if (solve(idx + 1)) return true;
            used[d] = false;
        }
        return false;
    }

    public static void main(String[] args) {
        leading.add(W1.charAt(0)); leading.add(W2.charAt(0)); leading.add(W3.charAt(0));
        Set<Character> seen = new HashSet<>();
        for (String w : new String[]{W1, W2, W3})
            for (char c : w.toCharArray())
                if (seen.add(c)) letters.add(c);
        solve(0);
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < letters.size(); i++) {
            char c = letters.get(i);
            sb.append("'").append(c).append("': ").append(assign.get(c)).append(i + 1 < letters.size() ? ", " : "");
        }
        sb.append("}");
        System.out.println(sb.toString());
    }
}

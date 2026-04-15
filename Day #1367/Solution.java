// Cryptarithmetic solver via backtracking over letter->digit assignments.
// Time O(10!/(10-L)!) worst with pruning, Space O(L). L = #distinct letters.
import java.util.*;

public class Solution {
    static String A, B, C;
    static List<Character> letters = new ArrayList<>();
    static Set<Character> leading = new HashSet<>();
    static Map<Character, Integer> assign = new LinkedHashMap<>();
    static boolean[] used = new boolean[10];

    static long val(String w) {
        long v = 0;
        for (char ch : w.toCharArray()) v = v * 10 + assign.get(ch);
        return v;
    }

    static boolean bt(int i) {
        if (i == letters.size()) return val(A) + val(B) == val(C);
        char ch = letters.get(i);
        for (int d = 0; d < 10; d++) {
            if (used[d]) continue;
            if (d == 0 && leading.contains(ch)) continue;
            used[d] = true; assign.put(ch, d);
            if (bt(i + 1)) return true;
            used[d] = false;
        }
        return false;
    }

    public static void main(String[] args) {
        A = "SEND"; B = "MORE"; C = "MONEY";
        Set<Character> seen = new HashSet<>();
        for (String w : new String[]{A, B, C}) {
            leading.add(w.charAt(0));
            for (char ch : w.toCharArray())
                if (seen.add(ch)) letters.add(ch);
        }
        if (bt(0)) {
            StringBuilder sb = new StringBuilder("{");
            for (int i = 0; i < letters.size(); i++) {
                char ch = letters.get(i);
                sb.append("'").append(ch).append("': ").append(assign.get(ch));
                if (i + 1 < letters.size()) sb.append(", ");
            }
            sb.append("}");
            System.out.println(sb);
        } else System.out.println("No solution");
    }
}

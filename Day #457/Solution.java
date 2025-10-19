// Day 457: All start indices in S that are anagrams of W.
// Fixed-size sliding window of char counts with match counter. Time O(|S|).
import java.util.ArrayList;
import java.util.List;
import java.util.StringJoiner;

public class Solution {
    static List<Integer> anagramIndices(String w, String s) {
        List<Integer> res = new ArrayList<>();
        int m = w.length(), n = s.length();
        if (m == 0 || m > n) return res;
        int[] need = new int[256], win = new int[256];
        for (char c : w.toCharArray()) need[c]++;
        for (int i = 0; i < n; i++) {
            win[s.charAt(i)]++;
            if (i >= m) win[s.charAt(i - m)]--;
            if (i >= m - 1) {
                boolean ok = true;
                for (int c = 0; c < 256; c++) if (need[c] != win[c]) { ok = false; break; }
                if (ok) res.add(i - m + 1);
            }
        }
        return res;
    }

    public static void main(String[] args) {
        StringJoiner sj = new StringJoiner(", ");
        for (int idx : anagramIndices("ab", "abxaba")) sj.add(String.valueOf(idx));
        System.out.println(sj); // 0, 3, 4
    }
}

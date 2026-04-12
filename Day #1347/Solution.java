// Reorganize string: count freqs, if max > (n+1)/2 impossible; sort chars by freq desc (tie by char),
// fill even indices first then odd. Time O(n log k), Space O(n).
import java.util.*;

public class Solution {
    static String reorganize(String s) {
        int n = s.length();
        int[] cnt = new int[128];
        for (char c : s.toCharArray()) cnt[c]++;
        for (int c = 0; c < 128; c++) if (cnt[c] > (n + 1) / 2) return null;
        List<Character> chars = new ArrayList<>();
        for (char c = 0; c < 128; c++) if (cnt[c] > 0) chars.add(c);
        chars.sort((a, b) -> {
            if (cnt[a] != cnt[b]) return cnt[b] - cnt[a]; // freq desc
            return a - b;                                 // tie by char asc
        });
        char[] res = new char[n];
        int idx = 0;
        for (char c : chars) {
            for (int k = 0; k < cnt[c]; k++) {
                res[idx] = c;
                idx += 2;
                if (idx >= n) idx = 1;
            }
        }
        return new String(res);
    }

    public static void main(String[] args) {
        String r1 = reorganize("aaabbc");
        System.out.println(r1 == null ? "None" : r1);
        String r2 = reorganize("aaab");
        System.out.println(r2 == null ? "None" : r2);
    }
}

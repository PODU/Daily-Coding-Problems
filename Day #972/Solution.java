// Day 972: Rearrange string so no two adjacent chars match (else "None").
// Approach: place most frequent chars into even-then-odd slots. Time O(n log k), Space O(n).
import java.util.*;

public class Solution {
    static String rearrange(String s) {
        int n = s.length();
        int[] cnt = new int[128];
        for (char c : s.toCharArray()) cnt[c]++;
        List<int[]> v = new ArrayList<>(); // {count, char}
        for (int c = 0; c < 128; c++) if (cnt[c] > 0) v.add(new int[]{cnt[c], c});
        v.sort((a, b) -> b[0] - a[0]);
        if (v.get(0)[0] > (n + 1) / 2) return "None";
        char[] res = new char[n];
        int idx = 0;
        for (int[] p : v) {
            for (int k = 0; k < p[0]; k++) {
                res[idx] = (char) p[1];
                idx += 2;
                if (idx >= n) idx = 1;
            }
        }
        return new String(res);
    }

    public static void main(String[] args) {
        System.out.println(rearrange("aaabbc")); // ababac
        System.out.println(rearrange("aaab"));   // None
    }
}

// Reorganize string: greedily place the most frequent remaining char that differs from the last.
// Max-heap by count. Time: O(n log A), Space: O(A).
import java.util.*;

public class Solution {
    static String reorganize(String s) {
        int[] cnt = new int[128];
        for (char c : s.toCharArray()) cnt[c]++;
        // highest count first; ties broken by smallest char for determinism
        PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[1] != b[1] ? b[1] - a[1] : a[0] - b[0]);
        for (char c = 0; c < 128; c++) if (cnt[c] > 0) pq.add(new int[]{c, cnt[c]});
        StringBuilder res = new StringBuilder();
        int[] prev = null;
        while (!pq.isEmpty()) {
            int[] cur = pq.poll();
            res.append((char) cur[0]);
            if (prev != null && prev[1] > 0) pq.add(prev);
            cur[1]--;
            prev = cur;
        }
        return res.length() == s.length() ? res.toString() : "None";
    }

    public static void main(String[] args) {
        System.out.println(reorganize("aaabbc")); // ababac (a valid arrangement)
        System.out.println(reorganize("aaab"));   // None
    }
}

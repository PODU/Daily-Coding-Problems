// Day 366: Rearrange so no two adjacent chars match (else null).
// Greedy with a max-heap by frequency; always place the most frequent char that
// isn't the one just placed. Feasible iff maxFreq <= (n+1)/2. Time O(n log k).
import java.util.*;

public class Solution {
    static String reorganize(String s) {
        int[] cnt = new int[128];
        for (char c : s.toCharArray()) cnt[c]++;
        PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> b[1] - a[1]);
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
        return res.length() == s.length() ? res.toString() : "null";
    }

    public static void main(String[] args) {
        System.out.println(reorganize("yyz"));  // yzy
        System.out.println(reorganize("yyy"));  // null
    }
}

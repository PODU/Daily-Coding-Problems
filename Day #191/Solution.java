// Day 191: Min intervals to remove = n - max non-overlapping set (touching allowed).
// Greedy by earliest end. Time O(n log n), Space O(1).
import java.util.*;

public class Solution {
    static int minRemovals(int[][] iv) {
        Arrays.sort(iv, (a, b) -> Integer.compare(a[1], b[1]));
        int kept = 0, end = Integer.MIN_VALUE;
        for (int[] p : iv)
            if (p[0] >= end) { kept++; end = p[1]; }
        return iv.length - kept;
    }

    public static void main(String[] args) {
        int[][] iv = {{7, 9}, {2, 4}, {5, 8}};
        System.out.println(minRemovals(iv));
    }
}

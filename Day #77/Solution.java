// Day 77: Merge overlapping intervals. Sort by start, then sweep merging.
// Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static int[][] mergeIntervals(int[][] iv) {
        Arrays.sort(iv, (a, b) -> Integer.compare(a[0], b[0]));
        List<int[]> res = new ArrayList<>();
        for (int[] p : iv) {
            if (!res.isEmpty() && p[0] <= res.get(res.size() - 1)[1])
                res.get(res.size() - 1)[1] = Math.max(res.get(res.size() - 1)[1], p[1]);
            else
                res.add(new int[]{p[0], p[1]});
        }
        return res.toArray(new int[0][]);
    }

    public static void main(String[] args) {
        int[][] in = {{1,3},{5,8},{4,10},{20,25}};
        int[][] res = mergeIntervals(in);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.length; i++) {
            sb.append("(").append(res[i][0]).append(", ").append(res[i][1]).append(")");
            if (i + 1 < res.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb); // [(1, 3), (4, 10), (20, 25)]
    }
}

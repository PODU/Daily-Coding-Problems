// Count attacking bishop pairs: group by diagonal (r-c) and anti-diagonal (r+c), sum c*(c-1)/2.
// Time: O(N), Space: O(N).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int M = 5;
        int[][] bishops = {{0,0},{1,2},{2,2},{4,0}};
        Map<Integer,Long> diag = new HashMap<>(), anti = new HashMap<>();
        for (int[] b : bishops) {
            diag.merge(b[0] - b[1], 1L, Long::sum);
            anti.merge(b[0] + b[1], 1L, Long::sum);
        }
        long pairs = 0;
        for (long c : diag.values()) pairs += c * (c - 1) / 2;
        for (long c : anti.values()) pairs += c * (c - 1) / 2;
        System.out.println(pairs);
    }
}

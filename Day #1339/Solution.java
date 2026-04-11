// Bishops attack along diagonals: group by (row-col) and (row+col); each group of k adds k*(k-1)/2. O(N) time, O(N) space.
import java.util.*;

public class Solution {
    static long countAttackingPairs(int M, int[][] bishops) {
        Map<Integer, Long> diag = new HashMap<>(), anti = new HashMap<>();
        for (int[] b : bishops) {
            diag.merge(b[0] - b[1], 1L, Long::sum);
            anti.merge(b[0] + b[1], 1L, Long::sum);
        }
        long pairs = 0;
        for (long k : diag.values()) pairs += k * (k - 1) / 2;
        for (long k : anti.values()) pairs += k * (k - 1) / 2;
        return pairs;
    }

    public static void main(String[] args) {
        int M = 5;
        int[][] bishops = {{0,0},{1,2},{2,2},{4,0}};
        System.out.println(countAttackingPairs(M, bishops));
    }
}

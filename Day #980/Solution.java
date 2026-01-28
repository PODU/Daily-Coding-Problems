// Count attacking bishop pairs by grouping on diagonals (row-col) and anti-diagonals (row+col).
// For each group of size k, add k*(k-1)/2. Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static long countAttackingPairs(int[][] bishops) {
        Map<Integer,Integer> diag = new HashMap<>(), anti = new HashMap<>();
        for (int[] b : bishops) {
            diag.merge(b[0] - b[1], 1, Integer::sum);
            anti.merge(b[0] + b[1], 1, Integer::sum);
        }
        long pairs = 0;
        for (int c : diag.values()) pairs += (long) c * (c - 1) / 2;
        for (int c : anti.values()) pairs += (long) c * (c - 1) / 2;
        return pairs;
    }

    public static void main(String[] args) {
        int[][] bishops = {{0,0},{1,2},{2,2},{4,0}};
        System.out.println(countAttackingPairs(bishops)); // 2
    }
}

// Count attacking bishop pairs: group by diagonals r+c and r-c, sum C(size,2).
// Time O(N), Space O(N).
import java.util.HashMap;
import java.util.Map;

public class Solution {
    static long countAttackingPairs(int[][] bishops) {
        Map<Integer, Long> diag1 = new HashMap<>(), diag2 = new HashMap<>();
        for (int[] b : bishops) {
            diag1.merge(b[0] + b[1], 1L, Long::sum);
            diag2.merge(b[0] - b[1], 1L, Long::sum);
        }
        long pairs = 0;
        for (long c : diag1.values()) pairs += c * (c - 1) / 2;
        for (long c : diag2.values()) pairs += c * (c - 1) / 2;
        return pairs;
    }

    public static void main(String[] args) {
        int[][] bishops = {{0, 0}, {1, 2}, {2, 2}, {4, 0}};
        System.out.println(countAttackingPairs(bishops));
    }
}

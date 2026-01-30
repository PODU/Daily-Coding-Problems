// Day 989: Deduce coin denominations from a ways-to-make-change array.
// Walk amounts; whenever target[i] exceeds reconstructed ways, i is a coin and we fold it into the DP.
// O(N^2) time, O(N) space.
import java.util.*;

public class Solution {
    static List<Integer> findDenominations(long[] target) {
        int n = target.length;
        long[] have = new long[n];
        have[0] = 1;                     // one way to make 0 with no coins
        List<Integer> coins = new ArrayList<>();
        for (int i = 1; i < n; i++) {
            if (target[i] > have[i]) {   // unaccounted combinations => i is a denomination
                coins.add(i);
                for (int j = i; j < n; j++) have[j] += have[j - i];
            }
        }
        return coins;
    }

    public static void main(String[] args) {
        long[] target = {1, 0, 1, 1, 2};
        List<Integer> coins = findDenominations(target);
        StringBuilder sb = new StringBuilder();
        for (int k = 0; k < coins.size(); k++) {
            sb.append(coins.get(k));
            if (k + 1 < coins.size()) sb.append(", ");
        }
        System.out.println(sb); // expected: 2, 3, 4
    }
}

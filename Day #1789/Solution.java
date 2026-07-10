// Min broadcast range: sort towers, binary-search nearest tower per listener, take max.
// Time O((N+M) log M), Space O(1) extra.
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[] listeners = {1, 5, 11, 20};
        int[] towers = {4, 8, 15};
        Arrays.sort(towers);
        int ans = 0;
        for (int L : listeners) {
            int pos = Arrays.binarySearch(towers, L);
            if (pos < 0) pos = -(pos + 1);
            int best = Integer.MAX_VALUE;
            if (pos < towers.length) best = Math.min(best, towers[pos] - L);
            if (pos > 0) best = Math.min(best, L - towers[pos - 1]);
            ans = Math.max(ans, best);
        }
        System.out.println(ans);
    }
}

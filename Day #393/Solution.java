// Largest consecutive range via hash set: from each start (n-1 absent) extend up. O(n) time, O(n) space.
import java.util.HashSet;
import java.util.Set;

public class Solution {
    static int[] largestRange(int[] nums) {
        Set<Integer> s = new HashSet<>();
        for (int n : nums) s.add(n);
        int bestLo = nums[0], bestHi = nums[0], bestLen = 0;
        for (int n : s) {
            if (s.contains(n - 1)) continue;
            int hi = n;
            while (s.contains(hi + 1)) hi++;
            if (hi - n + 1 > bestLen) { bestLen = hi - n + 1; bestLo = n; bestHi = hi; }
        }
        return new int[]{bestLo, bestHi};
    }

    public static void main(String[] args) {
        int[] nums = {9, 6, 1, 3, 8, 10, 12, 11};
        int[] r = largestRange(nums);
        System.out.println("(" + r[0] + ", " + r[1] + ")");
    }
}

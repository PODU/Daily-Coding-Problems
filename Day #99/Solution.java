// Day 99: Longest consecutive sequence. Hash all values; begin counting only at
// sequence starts (n-1 absent) and walk up. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static int longestConsecutive(int[] nums) {
        Set<Integer> s = new HashSet<>();
        for (int n : nums) s.add(n);
        int best = 0;
        for (int n : s) {
            if (!s.contains(n - 1)) {
                int length = 1;
                while (s.contains(n + length)) length++;
                best = Math.max(best, length);
            }
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(longestConsecutive(new int[]{100, 4, 200, 1, 3, 2})); // 4
    }
}

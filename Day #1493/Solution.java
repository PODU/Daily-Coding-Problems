// Detect a Pythagorean triplet a^2+b^2=c^2 in an array.
// Square + sort, then fix largest as c and two-pointer. Time O(n^2), Space O(1).
import java.util.Arrays;

public class Solution {
    static boolean hasTriplet(int[] nums) {
        long[] sq = new long[nums.length];
        for (int i = 0; i < nums.length; i++) sq[i] = (long) nums[i] * nums[i];
        Arrays.sort(sq);
        int n = sq.length;
        for (int c = n - 1; c >= 2; c--) {
            int a = 0, b = c - 1;
            while (a < b) {
                long s = sq[a] + sq[b];
                if (s == sq[c]) return true;
                if (s < sq[c]) a++; else b--;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        int[] nums = {3, 1, 4, 6, 5};
        System.out.println(hasTriplet(nums)); // expected: true
    }
}

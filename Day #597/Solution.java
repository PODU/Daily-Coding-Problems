// Day 597: Detect a Pythagorean triplet a^2 + b^2 = c^2 in an array.
// Approach: square values, sort, fix c as the largest, two-pointer. Time O(n^2), Space O(n).
import java.util.*;

public class Solution {
    static boolean hasPythagoreanTriplet(int[] nums) {
        long[] sq = new long[nums.length];
        for (int i = 0; i < nums.length; i++) sq[i] = (long) nums[i] * nums[i];
        Arrays.sort(sq);
        int n = sq.length;
        for (int c = n - 1; c >= 2; c--) {
            int lo = 0, hi = c - 1;
            while (lo < hi) {
                long s = sq[lo] + sq[hi];
                if (s == sq[c]) return true;
                else if (s < sq[c]) lo++;
                else hi--;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        int[] arr = {3, 1, 4, 6, 5};   // contains 3,4,5
        System.out.println(hasPythagoreanTriplet(arr));
    }
}

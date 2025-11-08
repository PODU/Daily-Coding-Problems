// Square a sorted array and return sorted. Two pointers from both ends pick larger
// absolute value into the back of the result. O(N) time, O(N) space.
import java.util.Arrays;

public class Solution {
    static long[] sortedSquares(int[] a) {
        int n = a.length, l = 0, r = n - 1;
        long[] res = new long[n];
        for (int k = n - 1; k >= 0; k--) {
            long lv = (long) a[l] * a[l], rv = (long) a[r] * a[r];
            if (lv > rv) { res[k] = lv; l++; }
            else { res[k] = rv; r--; }
        }
        return res;
    }

    public static void main(String[] args) {
        System.out.println(Arrays.toString(sortedSquares(new int[]{-9, -2, 0, 2, 3})));
        // [0, 4, 4, 9, 81]
    }
}

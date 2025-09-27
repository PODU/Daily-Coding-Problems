// Three entries summing to k: sort + fix one + two-pointer.
// O(n^2) time, O(1) extra space.
import java.util.*;

public class Solution {
    static boolean threeSum(int[] a, long k) {
        Arrays.sort(a);
        int n = a.length;
        for (int i = 0; i < n - 2; ++i) {
            int lo = i + 1, hi = n - 1;
            long target = k - a[i];
            while (lo < hi) {
                long s = (long) a[lo] + a[hi];
                if (s == target) return true;
                if (s < target) ++lo; else --hi;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        int[] a = {20, 303, 3, 4, 25};
        System.out.println(threeSum(a, 49)); // true
    }
}

// 3-sum decision: sort, fix one element, two-pointer scan the rest. O(n^2) time, O(1) extra.
import java.util.*;

public class Solution {
    static boolean threeSum(int[] a, int k) {
        Arrays.sort(a);
        int n = a.length;
        for (int i = 0; i < n - 2; i++) {
            int lo = i + 1, hi = n - 1;
            while (lo < hi) {
                long s = (long) a[i] + a[lo] + a[hi];
                if (s == k) return true;
                if (s < k) lo++; else hi--;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        System.out.println(threeSum(new int[]{20, 303, 3, 4, 25}, 49));
    }
}

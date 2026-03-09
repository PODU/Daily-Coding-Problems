// 3-sum existence: sort, fix each i, two-pointer scan remaining pair. Time O(N^2), Space O(1).
import java.util.*;

public class Solution {
    static boolean threeSum(int[] a, long k) {
        Arrays.sort(a);
        int n = a.length;
        for (int i = 0; i < n - 2; i++) {
            int lo = i + 1, hi = n - 1;
            while (lo < hi) {
                long s = (long) a[i] + a[lo] + a[hi];
                if (s == k) return true;
                else if (s < k) lo++;
                else hi--;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        int[] arr = {20, 303, 3, 4, 25};
        System.out.println(threeSum(arr, 49));
    }
}

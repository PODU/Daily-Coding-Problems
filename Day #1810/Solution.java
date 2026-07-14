// 3-sum decision: does any triple sum to k?
// Sort, then for each i two-pointer scan. Time: O(n^2). Space: O(1).
import java.util.Arrays;

public class Solution {
    static boolean threeSum(int[] a, int k) {
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
        System.out.println(threeSum(new int[]{20, 303, 3, 4, 25}, 49)); // true
    }
}

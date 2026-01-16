// Sort, then fix arr[i] and two-pointer scan for the remaining pair summing to k.
// Time O(n^2), Space O(1) extra.
import java.util.*;

public class Solution {
    static boolean threeSum(int[] arr, int k) {
        Arrays.sort(arr);
        int n = arr.length;
        for (int i = 0; i < n - 2; i++) {
            int lo = i + 1, hi = n - 1;
            while (lo < hi) {
                int s = arr[i] + arr[lo] + arr[hi];
                if (s == k) return true;
                if (s < k) lo++; else hi--;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        int[] arr = {20, 303, 3, 4, 25};
        System.out.println(threeSum(arr, 49) ? "true" : "false");
    }
}

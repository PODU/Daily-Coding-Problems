// Day 1181: Find the minimum in a rotated sorted array (no duplicates).
// Binary search: compare mid to the right end to discard the sorted half.
// Time O(log N), Space O(1).
public class Solution {
    static int findMin(int[] a) {
        int lo = 0, hi = a.length - 1;
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (a[mid] > a[hi]) lo = mid + 1;
            else hi = mid;
        }
        return a[lo];
    }

    public static void main(String[] args) {
        System.out.println(findMin(new int[]{5, 7, 10, 3, 4})); // 3
    }
}

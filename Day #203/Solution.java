// Day 203: Minimum of a rotated sorted array (no duplicates).
// Binary search: if mid > right, min is to the right; else min is at mid or left.
// Time: O(log n), Space: O(1).
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

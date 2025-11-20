// Day 636: Minimum in a rotated sorted array (no duplicates).
// Approach: binary search comparing mid with right endpoint.
// Time: O(log N), Space: O(1).
public class Solution {
    static int findMin(int[] a) {
        int lo = 0, hi = a.length - 1;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (a[mid] > a[hi]) lo = mid + 1;
            else hi = mid;
        }
        return a[lo];
    }

    public static void main(String[] args) {
        int[] a = {5, 7, 10, 3, 4};
        System.out.println(findMin(a)); // 3
    }
}

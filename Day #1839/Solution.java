// Day 1839: Minimum of a rotated sorted array (no duplicates) via binary search.
// Time O(log N), Space O(1).
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
        System.out.println(findMin(new int[]{5, 7, 10, 3, 4}));
    }
}

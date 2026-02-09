// Search in rotated sorted array via modified binary search.
// Time: O(log n), Space: O(1).
public class Solution {
    static Integer searchRotated(int[] a, int target) {
        int lo = 0, hi = a.length - 1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            if (a[mid] == target) return mid;
            if (a[lo] <= a[mid]) {
                if (a[lo] <= target && target < a[mid]) hi = mid - 1;
                else lo = mid + 1;
            } else {
                if (a[mid] < target && target <= a[hi]) lo = mid + 1;
                else hi = mid - 1;
            }
        }
        return null;
    }

    public static void main(String[] args) {
        int[] a = {13, 18, 25, 2, 8, 10};
        Integer idx = searchRotated(a, 8);
        System.out.println(idx == null ? "null" : idx); // 4
    }
}

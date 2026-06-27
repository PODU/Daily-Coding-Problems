// Day 1723: Search element in rotated sorted array.
// Modified binary search: one half is always sorted; decide which side to recurse.
// Time: O(log n), Space: O(1). Returns index, or -1 (null) if absent.
public class Solution {
    static int search(int[] a, int target) {
        int lo = 0, hi = a.length - 1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            if (a[mid] == target) return mid;
            if (a[lo] <= a[mid]) { // left half sorted
                if (a[lo] <= target && target < a[mid]) hi = mid - 1;
                else lo = mid + 1;
            } else { // right half sorted
                if (a[mid] < target && target <= a[hi]) lo = mid + 1;
                else hi = mid - 1;
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        int[] a = {13, 18, 25, 2, 8, 10};
        for (int target : new int[]{8, 99}) {
            int i = search(a, target);
            // the problem statement asks for null when the element is absent
            System.out.println(i >= 0 ? String.valueOf(i) : "null"); // 4, then null
        }
    }
}

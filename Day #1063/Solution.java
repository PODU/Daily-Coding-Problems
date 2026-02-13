// Day 1063: Find a peak in a rotated sorted array of distinct values.
// Approach: binary search; if a[mid] < a[mid+1] go right else left. Time O(log n), Space O(1).
public class Solution {
    static int findPeak(int[] a) {
        int lo = 0, hi = a.length - 1;
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (a[mid] < a[mid + 1]) lo = mid + 1;
            else hi = mid;
        }
        return lo; // index of the peak
    }

    public static void main(String[] args) {
        int[] a = {3, 4, 5, 1, 2};
        int idx = findPeak(a);
        System.out.println(a[idx]); // 5
    }
}

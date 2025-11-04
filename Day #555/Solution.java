// Search in rotated sorted array via modified binary search. O(log n) time, O(1) space.
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
        int[] arr = {13, 18, 25, 2, 8, 10};
        System.out.println(search(arr, 8));
        int miss = search(arr, 7);
        System.out.println(miss == -1 ? "null" : Integer.toString(miss));
    }
}

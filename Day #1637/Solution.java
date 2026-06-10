// Binary search without *, /, or bit-shift; midpoint via two-pointer walk (only ++/--).
// Time: O(log N), Space: O(1).
public class Solution {
    static int midpoint(int lo, int hi) {
        int i = lo, j = hi;
        while (i < j) { i++; j--; }
        return j; // floor((lo+hi)/2) using only ++/--
    }

    static boolean contains(int[] arr, int x) {
        int lo = 0, hi = arr.length - 1;
        while (lo <= hi) {
            int mid = midpoint(lo, hi);
            if (arr[mid] == x) return true;
            else if (arr[mid] < x) lo = mid + 1;
            else hi = mid - 1;
        }
        return false;
    }

    public static void main(String[] args) {
        int[] arr = {1, 3, 5, 7, 9, 11, 13};
        System.out.println(contains(arr, 7) ? "true" : "false");
        System.out.println(contains(arr, 8) ? "true" : "false");
    }
}

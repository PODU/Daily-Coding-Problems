// Find a peak in a rotated array (ends lowest) via binary search. O(log N) time, O(1) space.
public class Solution {
    static int findPeak(int[] a) {
        int lo = 0, hi = a.length - 1;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (a[mid] < a[mid + 1]) lo = mid + 1;
            else hi = mid;
        }
        return a[lo];
    }

    public static void main(String[] args) {
        int[] arr = {5, 7, 9, 3, 1};
        System.out.println(findPeak(arr));
    }
}

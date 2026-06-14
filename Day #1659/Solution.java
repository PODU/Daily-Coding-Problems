// Smallest unsorted window. Scan L->R tracking max for end, R->L tracking min for start. O(n) time, O(1) space.
public class Solution {
    static int[] unsortedWindow(int[] a) {
        int n = a.length, end = -1, start = -1;
        int mx = Integer.MIN_VALUE, mn = Integer.MAX_VALUE;
        for (int i = 0; i < n; i++) {
            mx = Math.max(mx, a[i]);
            if (a[i] < mx) end = i;
        }
        for (int i = n - 1; i >= 0; i--) {
            mn = Math.min(mn, a[i]);
            if (a[i] > mn) start = i;
        }
        return new int[]{start, end};
    }

    public static void main(String[] args) {
        int[] r = unsortedWindow(new int[]{3, 7, 5, 6, 9});
        System.out.println("(" + r[0] + ", " + r[1] + ")");
    }
}

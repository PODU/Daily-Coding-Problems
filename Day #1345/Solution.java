// Find smallest window to sort: right bound = last i where a[i] < running max; left bound = first j where a[j] > running min from right.
// Time: O(n), Space: O(1).
public class Solution {
    static int[] findUnsorted(int[] a) {
        int n = a.length;
        int end = -1, mx = Integer.MIN_VALUE;
        for (int i = 0; i < n; i++) {
            if (a[i] < mx) end = i;
            else mx = a[i];
        }
        int start = -1, mn = Integer.MAX_VALUE;
        for (int i = n - 1; i >= 0; i--) {
            if (a[i] > mn) start = i;
            else mn = a[i];
        }
        return new int[]{start, end};
    }

    public static void main(String[] args) {
        int[] a = {3, 7, 5, 6, 9};
        int[] r = findUnsorted(a);
        System.out.println("(" + r[0] + ", " + r[1] + ")");
    }
}

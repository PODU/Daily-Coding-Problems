// Day 257: Smallest window that must be sorted to make the whole array sorted.
// Left->right track max to find right bound; right->left track min to find left bound.
// Time: O(n), Space: O(1).
public class Solution {
    static int[] sortWindow(int[] a) {
        int n = a.length;
        int begin = -1, end = -1;
        int mx = Integer.MIN_VALUE;
        for (int i = 0; i < n; i++) {
            if (a[i] < mx) end = i;
            else mx = a[i];
        }
        int mn = Integer.MAX_VALUE;
        for (int i = n - 1; i >= 0; i--) {
            if (a[i] > mn) begin = i;
            else mn = a[i];
        }
        return new int[]{begin, end};
    }

    public static void main(String[] args) {
        int[] a = {3, 7, 5, 6, 9};
        int[] r = sortWindow(a);
        System.out.println("(" + r[0] + ", " + r[1] + ")"); // (1, 3)
    }
}

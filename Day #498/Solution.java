// Smallest window to sort so the whole array is sorted.
// Two passes: left->right track max for right bound; right->left track min for left bound.
// Time: O(n), Space: O(1).
public class Solution {
    static int[] windowToSort(int[] a) {
        int n = a.length;
        int left = -1, right = -1;
        int maxSoFar = a[0];
        for (int i = 1; i < n; i++) {
            if (a[i] < maxSoFar) right = i;
            else maxSoFar = a[i];
        }
        int minSoFar = a[n - 1];
        for (int i = n - 2; i >= 0; i--) {
            if (a[i] > minSoFar) left = i;
            else minSoFar = a[i];
        }
        return new int[]{left, right};
    }

    public static void main(String[] args) {
        int[] a = {3, 7, 5, 6, 9};
        int[] p = windowToSort(a);
        System.out.println("(" + p[0] + ", " + p[1] + ")");
    }
}

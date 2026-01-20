// Smallest window to sort: scan left->right tracking max for right bound,
// right->left tracking min for left bound. Time O(n), Space O(1).
public class Solution {
    static int[] findUnsortedWindow(int[] a) {
        int n = a.length;
        int right = -1, runMax = a[0];
        for (int i = 1; i < n; i++) {
            if (a[i] < runMax) right = i;
            else runMax = a[i];
        }
        int left = -1, runMin = a[n - 1];
        for (int i = n - 2; i >= 0; i--) {
            if (a[i] > runMin) left = i;
            else runMin = a[i];
        }
        return new int[]{left, right};
    }

    public static void main(String[] args) {
        int[] arr = {3, 7, 5, 6, 9};
        int[] r = findUnsortedWindow(arr);
        System.out.println("(" + r[0] + ", " + r[1] + ")");
    }
}

// First missing positive: cyclic sort (index-as-hash), place each x in [1,n] at index x-1.
// Time O(n), Space O(1) extra (in-place).
public class Solution {
    static int firstMissingPositive(int[] a) {
        int n = a.length;
        for (int i = 0; i < n; i++) {
            while (a[i] >= 1 && a[i] <= n && a[a[i] - 1] != a[i]) {
                int t = a[a[i] - 1];
                a[a[i] - 1] = a[i];
                a[i] = t;
            }
        }
        for (int i = 0; i < n; i++)
            if (a[i] != i + 1) return i + 1;
        return n + 1;
    }

    public static void main(String[] args) {
        System.out.println(firstMissingPositive(new int[]{3, 4, -1, 1})); // 2
        System.out.println(firstMissingPositive(new int[]{1, 2, 0}));     // 3
    }
}

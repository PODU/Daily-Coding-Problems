// Day 694: First missing positive integer in O(n) time, O(1) space.
// Approach: cyclic sort - place each value v in [1,n] at index v-1, then scan.
// Time O(n), Space O(1) (in-place).
public class Solution {
    static int firstMissingPositive(int[] a) {
        int n = a.length;
        for (int i = 0; i < n; i++)
            while (a[i] > 0 && a[i] <= n && a[a[i] - 1] != a[i]) {
                int j = a[i] - 1, t = a[j]; a[j] = a[i]; a[i] = t;
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

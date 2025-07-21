// First missing positive: place each value v in slot v-1 via swaps, then scan.
// Time: O(n), Space: O(1).
public class Solution {
    static int firstMissingPositive(int[] a) {
        int n = a.length;
        for (int i = 0; i < n; i++) {
            while (a[i] > 0 && a[i] <= n && a[a[i] - 1] != a[i]) {
                int j = a[i] - 1;
                int tmp = a[i]; a[i] = a[j]; a[j] = tmp;
            }
        }
        for (int i = 0; i < n; i++) if (a[i] != i + 1) return i + 1;
        return n + 1;
    }

    public static void main(String[] args) {
        System.out.println(firstMissingPositive(new int[]{3, 4, -1, 1}));
    }
}

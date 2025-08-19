// Nearest larger element's index by index-distance: expand outward from i. O(n) per query, O(1) space.

public class Solution {
    static Integer nearestLarger(int[] a, int i) {
        int n = a.length;
        for (int d = 1; d < n; d++) {
            if (i - d >= 0 && a[i - d] > a[i]) return i - d;
            if (i + d < n && a[i + d] > a[i]) return i + d;
        }
        return null;
    }

    public static void main(String[] args) {
        int[] a = {4, 1, 3, 5, 6};
        System.out.println(nearestLarger(a, 0)); // 3
    }
}

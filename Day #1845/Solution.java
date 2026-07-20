// Day 1845: Index of nearest larger value (by array distance) via outward expansion.
// Time O(N) per query, Space O(1). Returns null if none exists.
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
        System.out.println(nearestLarger(new int[]{4, 1, 3, 5, 6}, 0)); // 3
    }
}

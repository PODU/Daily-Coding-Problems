// Nearest larger: expand outward from i, return first index with greater value.
// Time: O(n), Space: O(1).
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
        Integer r = nearestLarger(a, 0);
        System.out.println(r == null ? "null" : r.toString());
    }
}

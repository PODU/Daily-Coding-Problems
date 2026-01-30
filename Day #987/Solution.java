// Day 987: Nearest larger number index.
// Expand outward from i checking i-d and i+d; first larger wins. O(n) per query, O(1) space.
// Follow-up: with O(n^2) preprocessing (or sparse tables) each query can be answered in O(1).
public class Solution {
    // Returns index of nearest larger element, or null if none.
    static Integer nearestLarger(int[] a, int i) {
        int n = a.length;
        for (int d = 1; d < n; d++) {
            int l = i - d, r = i + d;
            if (l >= 0 && a[l] > a[i]) return l; // prefer left on ties
            if (r < n && a[r] > a[i]) return r;
        }
        return null;
    }

    public static void main(String[] args) {
        int[] a = {4, 1, 3, 5, 6};
        Integer idx = nearestLarger(a, 0);
        System.out.println(idx == null ? "null" : idx); // expected 3
    }
}

// Expand outward from index i, returning nearest j (by |j-i|) with a[j] > a[i]; -1 if none.
// Time: O(n) per query, Space: O(1).
public class Solution {
    static int nearestLarger(int[] a, int i) {
        int n = a.length;
        for (int d = 1; d < n; d++) {
            int l = i - d, r = i + d;
            if (l >= 0 && a[l] > a[i]) return l;
            if (r < n && a[r] > a[i]) return r;
        }
        return -1;
    }

    public static void main(String[] args) {
        int[] a = {4, 1, 3, 5, 6};
        System.out.println(nearestLarger(a, 0));
    }
}

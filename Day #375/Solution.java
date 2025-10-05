// Day 375: h-index via counting sort.
// Bucket citations (capped at n), then scan h from n down accumulating papers
// with >= h citations; first h with count >= h wins. Time O(n), Space O(n).
public class Solution {
    static int hIndex(int[] cites) {
        int n = cites.length;
        int[] buckets = new int[n + 1];
        for (int c : cites) buckets[Math.min(c, n)]++;
        int total = 0;
        for (int h = n; h >= 0; h--) {
            total += buckets[h];
            if (total >= h) return h;
        }
        return 0;
    }

    public static void main(String[] args) {
        int[] cites = {4, 0, 0, 2, 3};
        System.out.println(hIndex(cites)); // 2
    }
}

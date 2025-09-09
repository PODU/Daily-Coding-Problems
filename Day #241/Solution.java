// h-index via counting/bucket sort. Time O(N), Space O(N).
// Bucket counts of citations capped at N, then scan from high to low.
public class Solution {
    static int hIndex(int[] citations) {
        int n = citations.length;
        int[] bucket = new int[n + 1];
        for (int c : citations) bucket[Math.min(c, n)]++;
        int total = 0;
        for (int h = n; h >= 0; h--) {
            total += bucket[h];
            if (total >= h) return h;
        }
        return 0;
    }

    public static void main(String[] args) {
        int[] citations = {4, 3, 0, 1, 5};
        System.out.println(hIndex(citations));
    }
}

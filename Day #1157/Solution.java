// H-index via counting buckets: bucket papers by min(citation, n), scan from high. O(n) time, O(n) space.
public class Solution {
    static int hIndex(int[] citations) {
        int n = citations.length;
        int[] bucket = new int[n + 1];
        for (int c : citations) bucket[Math.min(c, n)]++;
        int count = 0;
        for (int h = n; h >= 0; h--) {
            count += bucket[h];
            if (count >= h) return h;
        }
        return 0;
    }

    public static void main(String[] args) {
        int[] citations = {4, 3, 0, 1, 5};
        System.out.println(hIndex(citations));
    }
}

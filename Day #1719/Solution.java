// h-index via counting sort: bucket citations capped at N, scan from high to low
// accumulating papers until count >= citation level. Time O(N), Space O(N).
public class Solution {
    static int hIndex(int[] citations) {
        int n = citations.length;
        int[] bucket = new int[n + 1];
        for (int c : citations) bucket[Math.min(c, n)]++;
        int acc = 0;
        for (int h = n; h >= 0; --h) {
            acc += bucket[h];
            if (acc >= h) return h;
        }
        return 0;
    }

    public static void main(String[] args) {
        int[] citations = {4, 3, 0, 1, 5};
        System.out.println(hIndex(citations));
    }
}

// Day 428: Min cost to carve a strict pyramid (1,2,..,P,..,2,1); only lowering allowed.
// left[i]/right[i] cap each stone by distance from the ends; peak H = max min(left,right).
// A strict tent of peak H has sum H*H, so cost = sum(stones) - H*H. Time O(n), Space O(n).
public class Solution {
    public static void main(String[] args) {
        int[] stones = {1, 1, 3, 3, 2, 1};
        int n = stones.length;
        int[] left = new int[n], right = new int[n];
        left[0] = Math.min(stones[0], 1);
        for (int i = 1; i < n; i++) left[i] = Math.min(stones[i], left[i - 1] + 1);
        right[n - 1] = Math.min(stones[n - 1], 1);
        for (int i = n - 2; i >= 0; i--) right[i] = Math.min(stones[i], right[i + 1] + 1);
        int H = 0, p = 0;
        for (int i = 0; i < n; i++) {
            int b = Math.min(left[i], right[i]);
            if (b > H) { H = b; p = i; }
        }
        long total = 0;
        for (int x : stones) total += x;
        long cost = total - (long) H * H;
        StringBuilder sb = new StringBuilder();
        sb.append(cost).append("  (resulting in [");
        for (int i = 0; i < n; i++) {
            int h = Math.max(0, H - Math.abs(i - p));
            sb.append(h);
            if (i + 1 < n) sb.append(", ");
        }
        sb.append("])");
        System.out.println(sb.toString());
    }
}

// Day 269: Push dominoes simulation via force/two-pointer scan.
// Left-to-right add +force from R, right-to-left add -force from L, sign decides. Time O(n), Space O(n).
public class Solution {
    static String pushDominoes(String s) {
        int n = s.length();
        int[] force = new int[n];
        int f = 0;
        for (int i = 0; i < n; i++) {              // rightward push
            char c = s.charAt(i);
            if (c == 'R') f = n;
            else if (c == 'L') f = 0;
            else f = Math.max(f - 1, 0);
            force[i] += f;
        }
        f = 0;
        for (int i = n - 1; i >= 0; i--) {         // leftward push
            char c = s.charAt(i);
            if (c == 'L') f = n;
            else if (c == 'R') f = 0;
            else f = Math.max(f - 1, 0);
            force[i] -= f;
        }
        StringBuilder sb = new StringBuilder();
        for (int v : force)
            sb.append(v > 0 ? 'R' : v < 0 ? 'L' : '.');
        return sb.toString();
    }

    public static void main(String[] args) {
        System.out.println(pushDominoes(".L.R....L"));
        System.out.println(pushDominoes("..R...L.L"));
    }
}

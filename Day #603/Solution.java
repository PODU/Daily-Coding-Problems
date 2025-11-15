// Day 603: Final resting state of pushed dominoes.
// Approach: two force passes (R rightward, L leftward), compare. Time O(n), Space O(n).
public class Solution {
    static String pushDominoes(String s) {
        int n = s.length();
        int[] fR = new int[n], fL = new int[n];
        int f = 0;
        for (int i = 0; i < n; i++) {
            char c = s.charAt(i);
            if (c == 'R') f = n;
            else if (c == 'L') f = 0;
            else f = Math.max(f - 1, 0);
            fR[i] = f;
        }
        f = 0;
        for (int i = n - 1; i >= 0; i--) {
            char c = s.charAt(i);
            if (c == 'L') f = n;
            else if (c == 'R') f = 0;
            else f = Math.max(f - 1, 0);
            fL[i] = f;
        }
        char[] res = new char[n];
        for (int i = 0; i < n; i++) {
            if (fR[i] > fL[i]) res[i] = 'R';
            else if (fR[i] < fL[i]) res[i] = 'L';
            else res[i] = '.';
        }
        return new String(res);
    }

    public static void main(String[] args) {
        System.out.println(pushDominoes(".L.R....L")); // LL.RRRLLL
        System.out.println(pushDominoes("..R...L.L")); // ..RR.LLLL
    }
}

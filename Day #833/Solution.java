// Celebrity problem: one candidate via elimination, then verify.
// Two-pointer elimination + verification. Time: O(N) knows calls, Space: O(1).

public class Solution {
    // Demo knows matrix: N=4, person 2 is the celebrity.
    static final int[][] M = {
        {0, 1, 1, 0}, // 0 knows 2
        {0, 0, 1, 0}, // 1 knows 2
        {0, 0, 0, 0}, // 2 (celebrity) knows no one
        {0, 1, 1, 0}, // 3 knows 2
    };

    static boolean knows(int a, int b) { return M[a][b] == 1; }

    static int findCelebrity(int n) {
        int cand = 0;
        for (int i = 1; i < n; i++)
            if (knows(cand, i)) cand = i;
        for (int i = 0; i < n; i++) {
            if (i == cand) continue;
            if (knows(cand, i) || !knows(i, cand)) return -1;
        }
        return cand;
    }

    public static void main(String[] args) {
        System.out.println(findCelebrity(M.length));
    }
}

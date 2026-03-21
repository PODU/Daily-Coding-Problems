// Celebrity problem: O(N) elimination to one candidate, then O(N) verify.
public class Solution {
    static int[][] M;
    static boolean knows(int a, int b) { return M[a][b] == 1; }

    static int findCelebrity(int n) {
        int cand = 0;
        for (int i = 1; i < n; i++)
            if (knows(cand, i)) cand = i;
        for (int i = 0; i < n; i++)
            if (i != cand && (knows(cand, i) || !knows(i, cand))) return -1;
        return cand;
    }

    public static void main(String[] args) {
        M = new int[][]{{0, 1, 1, 0}, {0, 0, 1, 0}, {0, 0, 0, 0}, {0, 1, 1, 0}};
        System.out.println(findCelebrity(4));
    }
}

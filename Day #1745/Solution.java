// Celebrity finder: two-phase candidate elimination then verify. O(N) knows() calls, O(1) space.
public class Solution {
    static int[][] knowsMat = {
        {0, 1, 1, 0},
        {0, 0, 1, 0},
        {0, 0, 0, 0},
        {0, 1, 1, 0}
    };

    static boolean knows(int a, int b) { return knowsMat[a][b] == 1; }

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
        System.out.println(findCelebrity(4));
    }
}

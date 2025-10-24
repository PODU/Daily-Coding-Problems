// Day 486: Celebrity problem.
// Two-pointer elimination: one candidate survives in O(n) knows() calls, then
// verify in O(n). Total O(n) time, O(1) space.
public class Solution {
    // mock relation matrix: knows[a][b] == 1 means a knows b
    static int[][] KNOWS;

    static boolean knows(int a, int b) { return KNOWS[a][b] == 1; }

    static int findCelebrity(int n) {
        int candidate = 0;
        for (int i = 1; i < n; i++)
            if (knows(candidate, i)) candidate = i;
        for (int i = 0; i < n; i++) {
            if (i == candidate) continue;
            if (knows(candidate, i) || !knows(i, candidate)) return -1;
        }
        return candidate;
    }

    public static void main(String[] args) {
        // person 2 is the celebrity: knows nobody, everyone knows them
        KNOWS = new int[][]{
            {0, 1, 1, 0},
            {1, 0, 1, 1},
            {0, 0, 0, 0},
            {1, 1, 1, 0},
        };
        System.out.println(findCelebrity(4)); // 2
    }
}

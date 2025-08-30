// Day 195: In a row- and column-sorted matrix, count elements < M[i1,j1] or > M[i2,j2].
// Staircase counting of "<= x". Time O(N+M) per query, Space O(1).
// Note: the README example counts the lower bound inclusively (expected 15), so we use
// "<= M[i1,j1]" for the smaller side and strict "> M[i2,j2]" for the larger side.
public class Solution {
    static int countLessEqual(int[][] A, int x) {
        int n = A.length, m = A[0].length, r = 0, c = m - 1, cnt = 0;
        while (r < n && c >= 0) {
            if (A[r][c] <= x) { cnt += c + 1; r++; }
            else c--;
        }
        return cnt;
    }

    static int solve(int[][] A, int i1, int j1, int i2, int j2) {
        int n = A.length, m = A[0].length;
        int smaller = countLessEqual(A, A[i1][j1]);
        int larger = n * m - countLessEqual(A, A[i2][j2]);
        return smaller + larger;
    }

    public static void main(String[] args) {
        int[][] A = {
            {1, 3, 7, 10, 15, 20},
            {2, 6, 9, 14, 22, 25},
            {3, 8, 10, 15, 25, 30},
            {10, 11, 12, 23, 30, 35},
            {20, 25, 30, 35, 40, 45}};
        System.out.println(solve(A, 1, 1, 3, 3));
    }
}

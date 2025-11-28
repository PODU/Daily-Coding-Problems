// Day 668: Toeplitz matrix check. Every cell equals its top-left neighbor. Time O(m*n), Space O(1).
public class Solution {
    static boolean isToeplitz(int[][] m) {
        for (int i = 1; i < m.length; i++)
            for (int j = 1; j < m[i].length; j++)
                if (m[i][j] != m[i - 1][j - 1]) return false;
        return true;
    }

    public static void main(String[] args) {
        int[][] m = {
            {1, 2, 3, 4, 8},
            {5, 1, 2, 3, 4},
            {4, 5, 1, 2, 3},
            {7, 4, 5, 1, 2}};
        System.out.println(isToeplitz(m) ? "True" : "False"); // True
    }
}

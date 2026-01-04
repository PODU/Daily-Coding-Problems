// Day 855: count N-Queens solutions via backtracking with bitmasks for column/diagonals.
// O(N!) worst case, O(N) space. Bitmask makes conflict checks O(1).
public class Solution {
    static int full;
    static long count;

    static void solve(int cols, int diag1, int diag2){
        if(cols == full){ count++; return; }
        int avail = full & ~(cols | diag1 | diag2);
        while(avail != 0){
            int p = avail & (-avail);
            avail -= p;
            solve(cols | p, (diag1 | p) << 1, (diag2 | p) >> 1);
        }
    }

    static long countNQueens(int N){
        full = (1 << N) - 1; count = 0;
        solve(0, 0, 0);
        return count;
    }

    public static void main(String[] args){
        for(int N = 1; N <= 8; N++)
            System.out.println("N=" + N + ": " + countNQueens(N)); // N=8: 92
    }
}

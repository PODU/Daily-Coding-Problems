// Count all open knight's tours: DFS backtracking from every start square,
// counting Hamiltonian paths. Time O(8^(N*N)) worst, Space O(N*N). N=5 -> 1728.
public class Solution {
    static int N;
    static int[] dx = {1, 1, -1, -1, 2, 2, -2, -2};
    static int[] dy = {2, -2, 2, -2, 1, -1, 1, -1};

    static long dfs(int x, int y, int visited, boolean[][] board) {
        if (visited == N * N) return 1;
        long count = 0;
        for (int k = 0; k < 8; k++) {
            int nx = x + dx[k], ny = y + dy[k];
            if (nx >= 0 && nx < N && ny >= 0 && ny < N && !board[nx][ny]) {
                board[nx][ny] = true;
                count += dfs(nx, ny, visited + 1, board);
                board[nx][ny] = false;
            }
        }
        return count;
    }

    static long knightTours(int n) {
        N = n;
        if (n == 0) return 0;
        long total = 0;
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++) {
                boolean[][] board = new boolean[n][n];
                board[i][j] = true;
                total += dfs(i, j, 1, board);
            }
        return total;
    }

    public static void main(String[] args) {
        System.out.println(knightTours(5));
    }
}

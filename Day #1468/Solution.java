// Knight's tour counting via plain DFS backtracking from every start cell.
// Time: exponential (bounded by tour search); Space: O(N^2) visited grid + recursion.
public class Solution {
    static final int[] DX = {1, 1, -1, -1, 2, 2, -2, -2};
    static final int[] DY = {2, -2, 2, -2, 1, -1, 1, -1};
    static int N;
    static long total;
    static boolean[][] visited;

    static void dfs(int x, int y, int count) {
        if (count == N * N) { total++; return; }
        for (int d = 0; d < 8; d++) {
            int nx = x + DX[d], ny = y + DY[d];
            if (nx >= 0 && nx < N && ny >= 0 && ny < N && !visited[nx][ny]) {
                visited[nx][ny] = true;
                dfs(nx, ny, count + 1);
                visited[nx][ny] = false;
            }
        }
    }

    static long countTours(int n) {
        N = n; total = 0;
        visited = new boolean[n][n];
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++) {
                visited[i][j] = true;
                dfs(i, j, 1);
                visited[i][j] = false;
            }
        return total;
    }

    public static void main(String[] args) {
        System.out.println(countTours(1));
        System.out.println(countTours(5));
    }
}

// Day 704: Count valid Android unlock patterns of length N on a 3x3 keypad.
// Approach: DFS with a "skip" table (a jump is legal only if the middle key was
// already used); symmetry cuts the work. Time O(N!) worst but tiny (<=9 keys).
public class Solution {
    static int[][] skip = new int[10][10];
    static boolean[] visited = new boolean[10];

    static int dfs(int cur, int remaining) {
        if (remaining == 0) return 1;
        visited[cur] = true;
        int count = 0;
        for (int nx = 1; nx <= 9; nx++)
            if (!visited[nx]) {
                int mid = skip[cur][nx];
                if (mid == 0 || visited[mid]) count += dfs(nx, remaining - 1);
            }
        visited[cur] = false;
        return count;
    }

    static int numberOfPatterns(int N) {
        skip[1][3] = skip[3][1] = 2;
        skip[1][7] = skip[7][1] = 4;
        skip[3][9] = skip[9][3] = 6;
        skip[7][9] = skip[9][7] = 8;
        skip[1][9] = skip[9][1] = skip[3][7] = skip[7][3] = 5;
        skip[2][8] = skip[8][2] = skip[4][6] = skip[6][4] = 5;
        return 4 * dfs(1, N - 1) + 4 * dfs(2, N - 1) + dfs(5, N - 1);
    }

    public static void main(String[] args) {
        System.out.println(numberOfPatterns(4)); // 1624
    }
}

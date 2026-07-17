// Count knight's tours on NxN via DFS backtracking from every start cell.
// Worst-case exponential; fine for small N (N=5 -> 1728).
public class Solution {
    static int N;
    static int[] dr = {-2,-2,-1,-1,1,1,2,2};
    static int[] dc = {-1,1,-2,2,-2,2,-1,1};

    static long dfs(int r, int c, int count, boolean[][] vis){
        if(count == N*N) return 1;
        long total = 0;
        for(int k = 0; k < 8; k++){
            int nr = r + dr[k], nc = c + dc[k];
            if(nr>=0 && nr<N && nc>=0 && nc<N && !vis[nr][nc]){
                vis[nr][nc] = true;
                total += dfs(nr, nc, count+1, vis);
                vis[nr][nc] = false;
            }
        }
        return total;
    }

    static long countTours(int n){
        N = n;
        long total = 0;
        for(int r = 0; r < N; r++)
            for(int c = 0; c < N; c++){
                boolean[][] vis = new boolean[N][N];
                vis[r][c] = true;
                total += dfs(r, c, 1, vis);
            }
        return total;
    }

    public static void main(String[] args){
        System.out.println(countTours(5)); // 1728
    }
}

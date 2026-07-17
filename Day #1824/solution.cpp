// Count knight's tours on NxN via DFS backtracking from every start cell.
// Worst-case exponential; fine for small N (N=5 -> 1728).
#include <bits/stdc++.h>
using namespace std;

int N;
int dr[8] = {-2,-2,-1,-1,1,1,2,2};
int dc[8] = {-1,1,-2,2,-2,2,-1,1};

long long dfs(int r, int c, int count, vector<vector<bool>>& vis){
    if(count == N*N) return 1;
    long long total = 0;
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

long long countTours(int n){
    N = n;
    long long total = 0;
    for(int r = 0; r < N; r++)
        for(int c = 0; c < N; c++){
            vector<vector<bool>> vis(N, vector<bool>(N, false));
            vis[r][c] = true;
            total += dfs(r, c, 1, vis);
        }
    return total;
}

int main(){
    cout << countTours(5) << "\n"; // 1728
    return 0;
}

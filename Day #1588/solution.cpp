// Count right/down paths in a grid with walls. DP: dp[i][j] = paths to cell (0 if wall).
// Time: O(N*M); Space: O(N*M).
#include <bits/stdc++.h>
using namespace std;

long long countPaths(const vector<vector<int>>& grid){
    int n=grid.size(), m=grid[0].size();
    if(grid[0][0]==1||grid[n-1][m-1]==1) return 0;
    vector<vector<long long>> dp(n, vector<long long>(m,0));
    for(int i=0;i<n;i++){
        for(int j=0;j<m;j++){
            if(grid[i][j]==1){dp[i][j]=0;continue;}
            if(i==0&&j==0){dp[i][j]=1;continue;}
            long long up = (i>0)?dp[i-1][j]:0;
            long long left = (j>0)?dp[i][j-1]:0;
            dp[i][j]=up+left;
        }
    }
    return dp[n-1][m-1];
}

int main(){
    vector<vector<int>> grid={{0,0,1},{0,0,1},{1,0,0}};
    cout<<countPaths(grid)<<"\n";
    return 0;
}

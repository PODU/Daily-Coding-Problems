// Largest path value in a directed graph: DFS topological memo + color cycle detection.
// dp[u][c] = max count of letter c on a path ending at u. Cycle -> null. O((n+m)*26) time, O(n*26) space.
#include <bits/stdc++.h>
using namespace std;

vector<vector<int>> adj;
vector<array<int,26>> dp;
vector<int> state; // 0 unvisited, 1 in-stack, 2 done
string colors;

bool dfs(int u){
    state[u]=1;
    for(int v:adj[u]){
        if(state[v]==1) return false;        // back edge -> cycle
        if(state[v]==0){ if(!dfs(v)) return false; }
    }
    for(int v:adj[u])
        for(int c=0;c<26;c++) dp[u][c]=max(dp[u][c],dp[v][c]);
    dp[u][colors[u]-'A']++;
    state[u]=2;
    return true;
}

int largestPathValue(const string& cols, vector<pair<int,int>>& edges){
    int n=cols.size();
    colors=cols;
    adj.assign(n,{});
    for(auto&e:edges) adj[e.first].push_back(e.second);
    dp.assign(n,{});
    state.assign(n,0);
    for(int i=0;i<n;i++)
        if(state[i]==0 && !dfs(i)) return -1; // -1 signals cycle (null)
    int ans=0;
    for(int i=0;i<n;i++) for(int c=0;c<26;c++) ans=max(ans,dp[i][c]);
    return ans;
}

int main(){
    string c1="ABACA";
    vector<pair<int,int>> e1={{0,1},{0,2},{2,3},{3,4}};
    int r1=largestPathValue(c1,e1);
    if(r1<0) cout<<"null\n"; else cout<<r1<<"\n";

    string c2="A";
    vector<pair<int,int>> e2={{0,0}};
    int r2=largestPathValue(c2,e2);
    if(r2<0) cout<<"null\n"; else cout<<r2<<"\n";
    return 0;
}

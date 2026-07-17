// Word search: DFS backtracking from each cell. O(R*C*4^L) time, O(L) space.
#include <bits/stdc++.h>
using namespace std;

bool dfs(vector<vector<char>>& b, const string& w, int i, int r, int c){
    if(i == (int)w.size()) return true;
    if(r < 0 || r >= (int)b.size() || c < 0 || c >= (int)b[0].size() || b[r][c] != w[i])
        return false;
    char saved = b[r][c];
    b[r][c] = '#';
    bool found = dfs(b,w,i+1,r+1,c) || dfs(b,w,i+1,r-1,c) ||
                 dfs(b,w,i+1,r,c+1) || dfs(b,w,i+1,r,c-1);
    b[r][c] = saved;
    return found;
}

bool exists(vector<vector<char>> b, const string& w){
    for(int r = 0; r < (int)b.size(); r++)
        for(int c = 0; c < (int)b[0].size(); c++)
            if(dfs(b, w, 0, r, c)) return true;
    return false;
}

int main(){
    vector<vector<char>> board = {
        {'A','B','C','E'},
        {'S','F','C','S'},
        {'A','D','E','E'}
    };
    for(string w : {"ABCCED","SEE","ABCB"})
        cout << "exists(board, \"" << w << "\") returns "
             << (exists(board, w) ? "true" : "false") << "\n";
    return 0;
}

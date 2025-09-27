// Max non-overlapping dictionary words on a board.
// (1) DFS enumerate every placement (bitmask of cells) per word. (2) Backtrack over
// placements choosing pairwise-disjoint sets to maximize count. ~O(placements * 2^?) pruned.
#include <bits/stdc++.h>
using namespace std;

int R, C;
vector<string> dict;
vector<vector<char>> board;

void dfs(const string& w, int idx, int r, int c, int used, set<int>& masks){
    if(board[r][c] != w[idx]) return;
    int cell = r*C + c;
    if(used & (1<<cell)) return;
    used |= (1<<cell);
    if(idx == (int)w.size()-1){ masks.insert(used); return; }
    int dr[]={-1,1,0,0}, dc[]={0,0,-1,1};
    for(int k=0;k<4;k++){
        int nr=r+dr[k], nc=c+dc[k];
        if(nr>=0&&nr<R&&nc>=0&&nc<C) dfs(w, idx+1, nr, nc, used, masks);
    }
}

// placements: list of (wordIndex, mask)
vector<pair<int,int>> placements;
int bestCount;

void backtrack(int start, int occupied, int usedWordsMask, int count){
    bestCount = max(bestCount, count);
    for(int i=start;i<(int)placements.size();i++){
        int w = placements[i].first, m = placements[i].second;
        if(usedWordsMask & (1<<w)) continue;
        if(occupied & m) continue;
        backtrack(i+1, occupied|m, usedWordsMask|(1<<w), count+1);
    }
}

int main(){
    dict = {"eat","rain","in","rat"};
    board = {{'e','a','n'},{'t','t','i'},{'a','r','a'}};
    R = board.size(); C = board[0].size();
    for(int wi=0; wi<(int)dict.size(); wi++){
        set<int> masks;
        for(int r=0;r<R;r++) for(int c=0;c<C;c++) dfs(dict[wi], 0, r, c, 0, masks);
        for(int m: masks) placements.push_back({wi, m});
    }
    bestCount = 0;
    backtrack(0, 0, 0, 0);
    cout << bestCount << endl;
    return 0;
}

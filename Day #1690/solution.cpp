// Boggle solver: build a trie from the dictionary, DFS 8-directionally from each
// cell following trie edges with a visited mask. O(cells * 8^L) worst, pruned by trie.
#include <iostream>
#include <vector>
#include <set>
#include <string>
#include <functional>
using namespace std;

struct TrieNode { TrieNode* ch[26]={}; bool end=false; };

int main(){
    vector<string> grid = {"oaan","etae","ihkr","iflv"};
    vector<string> dict = {"oath","pea","eat","rain"};

    TrieNode* root=new TrieNode();
    for(const string& w:dict){
        TrieNode* nd=root;
        for(char c:w){ int i=c-'a'; if(!nd->ch[i]) nd->ch[i]=new TrieNode(); nd=nd->ch[i]; }
        nd->end=true;
    }

    int R=(int)grid.size(), C=(int)grid[0].size();
    set<string> found;
    vector<vector<bool>> vis(R, vector<bool>(C,false));

    function<void(int,int,TrieNode*,string&)> dfs=[&](int r,int c,TrieNode* node,string& path){
        int idx=grid[r][c]-'a';
        TrieNode* nxt=node->ch[idx];
        if(!nxt) return;
        path.push_back(grid[r][c]);
        if(nxt->end) found.insert(path);
        vis[r][c]=true;
        for(int dr=-1;dr<=1;dr++) for(int dc=-1;dc<=1;dc++){
            if(dr==0&&dc==0) continue;
            int nr=r+dr,nc=c+dc;
            if(nr>=0&&nr<R&&nc>=0&&nc<C&&!vis[nr][nc]) dfs(nr,nc,nxt,path);
        }
        vis[r][c]=false;
        path.pop_back();
    };

    for(int r=0;r<R;r++) for(int c=0;c<C;c++){ string path=""; dfs(r,c,root,path); }

    for(const string& w:found) cout<<w<<"\n";
    return 0;
}

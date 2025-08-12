// Day 110: Root-to-leaf paths via DFS backtracking. O(n) nodes, O(h) stack.
#include <bits/stdc++.h>
using namespace std;
struct Node { int val; Node *l, *r; Node(int v):val(v),l(nullptr),r(nullptr){} };

void dfs(Node* n, vector<int>& path, vector<vector<int>>& res){
    if(!n) return;
    path.push_back(n->val);
    if(!n->l && !n->r) res.push_back(path);
    else { dfs(n->l, path, res); dfs(n->r, path, res); }
    path.pop_back();
}
int main(){
    Node* root = new Node(1);
    root->l = new Node(2);
    root->r = new Node(3);
    root->r->l = new Node(4);
    root->r->r = new Node(5);
    vector<vector<int>> res; vector<int> path;
    dfs(root, path, res);
    cout << "[";
    for(size_t i=0;i<res.size();++i){
        cout << "[";
        for(size_t j=0;j<res[i].size();++j) cout << res[i][j] << (j+1<res[i].size()?", ":"");
        cout << "]" << (i+1<res.size()?", ":"");
    }
    cout << "]\n";
    return 0;
}

// Largest BST subtree size via post-order DFS returning {isBST,size,min,max}. O(n) time.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *l=nullptr,*r=nullptr; Node(int v):val(v){} };

struct Info { bool isBST; int size; int mn; int mx; };

int best = 0;
Info dfs(Node* n){
    if(!n) return {true, 0, INT_MAX, INT_MIN};
    Info L = dfs(n->l), R = dfs(n->r);
    if(L.isBST && R.isBST && n->val > L.mx && n->val < R.mn){
        int sz = L.size + R.size + 1;
        best = max(best, sz);
        return {true, sz, min(n->val, L.mn), max(n->val, R.mx)};
    }
    return {false, 0, 0, 0};
}

int main(){
    Node* root = new Node(10);
    root->l = new Node(5); root->r = new Node(15);
    root->l->l = new Node(1); root->l->r = new Node(8);
    root->r->r = new Node(7);
    dfs(root);
    cout << best << "\n";
    return 0;
}

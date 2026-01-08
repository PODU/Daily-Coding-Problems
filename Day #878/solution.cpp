// Largest BST subtree via post-order returning (isBST, size, min, max). Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v): val(v), left(nullptr), right(nullptr) {}
};

struct Info { bool isBST; int size, mn, mx; };

int best = 0;

Info dfs(Node* n){
    if(!n) return {true, 0, INT_MAX, INT_MIN};
    Info l = dfs(n->left), r = dfs(n->right);
    if(l.isBST && r.isBST && n->val > l.mx && n->val < r.mn){
        int sz = l.size + r.size + 1;
        best = max(best, sz);
        return {true, sz, min(n->val, l.mn), max(n->val, r.mx)};
    }
    return {false, 0, 0, 0};
}

int largestBST(Node* root){
    best = 0;
    dfs(root);
    return best;
}

int main(){
    Node* root = new Node(10);
    root->left = new Node(5); root->right = new Node(15);
    root->left->left = new Node(1); root->left->right = new Node(8);
    root->right->right = new Node(7);
    cout << largestBST(root) << "\n";
    return 0;
}

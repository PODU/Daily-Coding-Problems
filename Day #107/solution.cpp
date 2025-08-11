// Day 107: Level-order (BFS) traversal of a binary tree. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;
struct Node { int val; Node *l, *r; Node(int v):val(v),l(nullptr),r(nullptr){} };

vector<int> levelOrder(Node* root){
    vector<int> out;
    if(!root) return out;
    queue<Node*> q; q.push(root);
    while(!q.empty()){
        Node* n = q.front(); q.pop();
        out.push_back(n->val);
        if(n->l) q.push(n->l);
        if(n->r) q.push(n->r);
    }
    return out;
}
int main(){
    Node* root = new Node(1);
    root->l = new Node(2);
    root->r = new Node(3);
    root->r->l = new Node(4);
    root->r->r = new Node(5);
    auto v = levelOrder(root);
    for(size_t i=0;i<v.size();++i) cout << v[i] << (i+1<v.size()?", ":"\n");
    return 0;
}

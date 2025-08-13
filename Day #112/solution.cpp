// Day 112: LCA with parent pointers - equalize depths, walk up together. O(h).
#include <bits/stdc++.h>
using namespace std;
struct Node { int val; Node *parent=nullptr, *l=nullptr, *r=nullptr; Node(int v):val(v){} };
Node* mk(Node* p, int v){ Node* n=new Node(v); n->parent=p; return n; }
int depth(Node* n){ int d=0; while(n){ n=n->parent; d++; } return d; }

Node* lca(Node* a, Node* b){
    int da=depth(a), db=depth(b);
    while(da>db){ a=a->parent; da--; }
    while(db>da){ b=b->parent; db--; }
    while(a!=b){ a=a->parent; b=b->parent; }
    return a;
}
int main(){
    Node* root=new Node(3);
    root->l=mk(root,5); root->r=mk(root,1);
    root->l->l=mk(root->l,6); root->l->r=mk(root->l,2);
    root->r->l=mk(root->r,0); root->r->r=mk(root->r,8);
    root->l->r->l=mk(root->l->r,7); root->l->r->r=mk(root->l->r,4);
    cout << lca(root->l, root->r)->val << "\n";            // LCA(5,1)=3
    cout << lca(root->l->r->l, root->l->r->r)->val << "\n"; // LCA(7,4)=2
    return 0;
}

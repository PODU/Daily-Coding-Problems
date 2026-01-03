// Day 842: invert (mirror) a binary tree by swapping children at every node.
// Recursive DFS. O(n) time, O(h) stack space.
#include <bits/stdc++.h>
using namespace std;

struct Node { char v; Node *l, *r; Node(char c): v(c), l(nullptr), r(nullptr) {} };

Node* invert(Node* root){
    if(!root) return nullptr;
    swap(root->l, root->r);
    invert(root->l);
    invert(root->r);
    return root;
}

void levelOrder(Node* root){
    if(!root) return;
    queue<Node*> q; q.push(root);
    vector<char> out;
    while(!q.empty()){
        Node* n = q.front(); q.pop();
        out.push_back(n->v);
        if(n->l) q.push(n->l);
        if(n->r) q.push(n->r);
    }
    for(size_t i = 0; i < out.size(); ++i){
        cout << out[i];
        if(i + 1 < out.size()) cout << ' ';
    }
    cout << "\n";
}

int main(){
    Node *a=new Node('a'),*b=new Node('b'),*c=new Node('c'),
         *d=new Node('d'),*e=new Node('e'),*f=new Node('f');
    a->l=b; a->r=c; b->l=d; b->r=e; c->l=f;
    invert(a);
    levelOrder(a); // a c b f e d
    return 0;
}

// Convert to full binary tree by removing single-child nodes (post-order recursion).
// O(N) time, O(H) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *l, *r; Node(int v):val(v),l(nullptr),r(nullptr){} };

Node* fullify(Node* n){
    if(!n) return nullptr;
    n->l = fullify(n->l);
    n->r = fullify(n->r);
    if(!n->l && n->r) return n->r;
    if(n->l && !n->r) return n->l;
    return n;
}

string serialize(Node* n){
    if(!n) return "";
    if(!n->l && !n->r) return to_string(n->val);
    return to_string(n->val) + "(" + serialize(n->l) + ", " + serialize(n->r) + ")";
}

int main(){
    Node* n0=new Node(0); Node* n1=new Node(1); Node* n2=new Node(2);
    Node* n3=new Node(3); Node* n4=new Node(4); Node* n5=new Node(5);
    Node* n6=new Node(6); Node* n7=new Node(7);
    n0->l=n1; n0->r=n2;
    n1->l=n3;
    n2->r=n4;
    n3->r=n5;
    n4->l=n6; n4->r=n7;
    Node* root = fullify(n0);
    cout << serialize(root) << "\n"; // 0(5, 4(6, 7))
    return 0;
}

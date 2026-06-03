// BST floor (largest <= x) & ceiling (smallest >= x). Iterative O(h) time, O(1) space.
// Floor: go right recording when val<=x else left. Ceiling: symmetric.
#include <iostream>
#include <string>
using namespace std;

struct Node { int val; Node *l=nullptr,*r=nullptr; Node(int v):val(v){} };

Node* insert(Node* root, int v) {
    if (!root) return new Node(v);
    if (v < root->val) root->l = insert(root->l, v);
    else root->r = insert(root->r, v);
    return root;
}

// returns pair<found, value>
pair<bool,int> floorBST(Node* root, int x) {
    bool f=false; int res=0;
    while (root) {
        if (root->val == x) return {true, x};
        if (root->val < x) { f=true; res=root->val; root=root->r; }
        else root=root->l;
    }
    return {f,res};
}

pair<bool,int> ceilBST(Node* root, int x) {
    bool f=false; int res=0;
    while (root) {
        if (root->val == x) return {true, x};
        if (root->val > x) { f=true; res=root->val; root=root->l; }
        else root=root->r;
    }
    return {f,res};
}

string show(pair<bool,int> p){ return p.first ? to_string(p.second) : "None"; }

void query(Node* root, int x){
    cout << "x=" << x << " -> floor " << show(floorBST(root,x))
         << ", ceiling " << show(ceilBST(root,x)) << endl;
}

int main(){
    Node* root=nullptr;
    for (int v : {8,4,12,2,6,10,14}) root=insert(root,v);
    query(root,5);   // floor 4, ceiling 6
    query(root,8);   // floor 8, ceiling 8
    query(root,15);  // floor 14, ceiling None
    query(root,1);   // floor None, ceiling 2
    return 0;
}

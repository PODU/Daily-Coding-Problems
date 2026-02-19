// Day 1098: Floor and ceiling of x in a BST.
// Walk down the tree updating candidates using BST ordering.
// Time: O(h). Space: O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *l=nullptr,*r=nullptr; Node(int v):val(v){} };

Node* insert(Node* root, int v){
    if (!root) return new Node(v);
    if (v < root->val) root->l = insert(root->l, v);
    else root->r = insert(root->r, v);
    return root;
}

// floor = largest value <= x, ceil = smallest value >= x; INT_MIN/MAX means none.
pair<long long,long long> floorCeil(Node* root, int x){
    long long fl = LLONG_MIN, ce = LLONG_MAX;
    Node* cur = root;
    while (cur){
        if (cur->val == x) return {x, x};
        if (cur->val < x){ fl = cur->val; cur = cur->r; }
        else { ce = cur->val; cur = cur->l; }
    }
    return {fl, ce};
}

int main(){
    Node* root=nullptr;
    for (int v : {8,4,12,2,6,10,14}) root = insert(root, v);
    auto pr = floorCeil(root, 5);
    cout << "floor=" << (pr.first==LLONG_MIN?string("None"):to_string(pr.first))
         << " ceil=" << (pr.second==LLONG_MAX?string("None"):to_string(pr.second)) << "\n"; // floor=4 ceil=6
    return 0;
}

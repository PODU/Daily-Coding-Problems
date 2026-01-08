// Subtree check: for each node of s, test identical-tree with t. Time O(m*n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v): val(v), left(nullptr), right(nullptr) {}
};

bool sameTree(Node* a, Node* b){
    if(!a && !b) return true;
    if(!a || !b) return false;
    return a->val == b->val && sameTree(a->left, b->left) && sameTree(a->right, b->right);
}

bool isSubtree(Node* s, Node* t){
    if(!s) return false;
    if(sameTree(s, t)) return true;
    return isSubtree(s->left, t) || isSubtree(s->right, t);
}

int main(){
    Node* s = new Node(3);
    s->left = new Node(4); s->right = new Node(5);
    s->left->left = new Node(1); s->left->right = new Node(2);

    Node* t = new Node(4);
    t->left = new Node(1); t->right = new Node(2);

    cout << (isSubtree(s, t) ? "true" : "false") << "\n";
    return 0;
}

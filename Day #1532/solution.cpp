// Invert (mirror) a binary tree by swapping left/right children of every node.
// Time O(n), Space O(h) recursion.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char val;
    Node *l, *r;
    Node(char v) : val(v), l(nullptr), r(nullptr) {}
};

Node* invert(Node* root) {
    if (!root) return nullptr;
    swap(root->l, root->r);
    invert(root->l);
    invert(root->r);
    return root;
}

void preorder(Node* root, string& out) {
    if (!root) return;
    if (!out.empty()) out += ' ';
    out += root->val;
    preorder(root->l, out);
    preorder(root->r, out);
}

int main() {
    Node* a = new Node('a');
    a->l = new Node('b');
    a->r = new Node('c');
    a->l->l = new Node('d');
    a->l->r = new Node('e');
    a->r->l = new Node('f');
    string before, after;
    preorder(a, before);
    invert(a);
    preorder(a, after);
    cout << "before (preorder): " << before << "\n";
    cout << "after  (preorder): " << after << "\n";
    return 0;
}

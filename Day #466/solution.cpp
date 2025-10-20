// Symmetric k-ary tree: recursively compare children of two subtrees in mirror order.
// Time: O(n), Space: O(h) recursion.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    vector<Node*> children;
    Node(int v) : val(v) {}
};

bool mirror(Node* a, Node* b) {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (a->val != b->val) return false;
    if (a->children.size() != b->children.size()) return false;
    int n = a->children.size();
    for (int i = 0; i < n; i++)
        if (!mirror(a->children[i], b->children[n - 1 - i])) return false;
    return true;
}

bool isSymmetric(Node* root) {
    if (!root) return true;
    return mirror(root, root);
}

int main() {
    Node* root = new Node(4);
    Node* l3 = new Node(3), *m5 = new Node(5), *r3 = new Node(3);
    root->children = {l3, m5, r3};
    l3->children = {new Node(9)};
    r3->children = {new Node(9)};
    cout << (isSymmetric(root) ? "True" : "False") << endl;
    return 0;
}

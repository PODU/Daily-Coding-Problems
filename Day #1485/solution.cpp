// Day 1485: Determine whether a k-ary tree is symmetric about its root.
// Subtrees mirror iff values match and child i mirrors child (k-1-i).
// Time O(N), Space O(H).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    vector<Node*> children;
    Node(int v, vector<Node*> c = {}) : val(v), children(c) {}
};

bool isMirror(Node* a, Node* b) {
    if (a->val != b->val || a->children.size() != b->children.size()) return false;
    int k = a->children.size();
    for (int i = 0; i < k; ++i)
        if (!isMirror(a->children[i], b->children[k - 1 - i])) return false;
    return true;
}

bool isSymmetric(Node* root) {
    return root == nullptr || isMirror(root, root);
}

int main() {
    Node* tree = new Node(4, {
        new Node(3, {new Node(9)}),
        new Node(5),
        new Node(3, {new Node(9)}),
    });
    cout << (isSymmetric(tree) ? "True" : "False") << "\n";  // True
}

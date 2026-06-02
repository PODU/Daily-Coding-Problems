// Approach: Symmetric k-ary tree via isMirror recursion comparing children mirror-wise.
// Time O(n), Space O(h) recursion.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    vector<Node*> children;
    Node(int v): val(v) {}
};

bool isMirror(Node* a, Node* b) {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (a->val != b->val) return false;
    if (a->children.size() != b->children.size()) return false;
    int k = a->children.size();
    for (int i = 0; i < k; i++)
        if (!isMirror(a->children[i], b->children[k - 1 - i])) return false;
    return true;
}

bool isSymmetric(Node* root) {
    if (!root) return true;
    return isMirror(root, root);
}

int main() {
    Node* root = new Node(4);
    Node* c1 = new Node(3); c1->children.push_back(new Node(9));
    Node* c2 = new Node(5);
    Node* c3 = new Node(3); c3->children.push_back(new Node(9));
    root->children = {c1, c2, c3};
    cout << (isSymmetric(root) ? "true" : "false") << "\n";
    return 0;
}

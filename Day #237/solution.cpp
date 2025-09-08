// Symmetric k-ary tree: a tree is symmetric iff left subtree mirrors right subtree.
// Recursively compare children in mirrored order. Time: O(N), Space: O(height).
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
    int n = root->children.size();
    for (int i = 0; i < n / 2; i++)
        if (!mirror(root->children[i], root->children[n - 1 - i])) return false;
    return true;
}

int main() {
    Node* root = new Node(4);
    Node* a = new Node(3); a->children.push_back(new Node(9));
    Node* b = new Node(5);
    Node* c = new Node(3); c->children.push_back(new Node(9));
    root->children = {a, b, c};
    cout << boolalpha << isSymmetric(root) << "\n"; // true
}

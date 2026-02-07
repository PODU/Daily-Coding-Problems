// Sorted array -> height-balanced BST: recursively pick middle as root.
// Time: O(n), Space: O(log n) recursion.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* left;
    Node* right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

Node* build(const vector<int>& a, int lo, int hi) {
    if (lo > hi) return nullptr;
    int mid = lo + (hi - lo) / 2;
    Node* root = new Node(a[mid]);
    root->left = build(a, lo, mid - 1);
    root->right = build(a, mid + 1, hi);
    return root;
}

void printRotated(Node* node, int depth) {
    if (!node) return;
    printRotated(node->right, depth + 1);
    cout << string(depth * 4, ' ') << node->val << "\n";
    printRotated(node->left, depth + 1);
}

void inorder(Node* node, vector<int>& out) {
    if (!node) return;
    inorder(node->left, out);
    out.push_back(node->val);
    inorder(node->right, out);
}

int main() {
    vector<int> a = {-10, -3, 0, 5, 9};
    Node* root = build(a, 0, (int)a.size() - 1);
    cout << "Height-balanced BST (rotated 90 deg):\n";
    printRotated(root, 0);
    vector<int> io;
    inorder(root, io);
    cout << "In-order:";
    for (int x : io) cout << " " << x;
    cout << "\n";
    return 0;
}

// Day 1325: Sorted array -> height-balanced BST.
// Recursively pick the middle element as the root so both halves differ in height by <=1. O(n) time, O(log n) stack.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v): val(v), left(nullptr), right(nullptr) {} };

Node* build(const vector<int>& a, int lo, int hi) {
    if (lo > hi) return nullptr;
    int mid = lo + (hi - lo) / 2;
    Node* root = new Node(a[mid]);
    root->left = build(a, lo, mid - 1);
    root->right = build(a, mid + 1, hi);
    return root;
}

void preorder(Node* n, vector<int>& out) {
    if (!n) return;
    out.push_back(n->val);
    preorder(n->left, out);
    preorder(n->right, out);
}

int main() {
    vector<int> a = {1, 2, 3, 4, 5, 6, 7};
    Node* root = build(a, 0, a.size() - 1);
    vector<int> out;
    preorder(root, out);
    cout << "preorder: ";
    for (int v : out) cout << v << " ";
    cout << endl; // preorder: 4 2 1 3 6 5 7
    return 0;
}

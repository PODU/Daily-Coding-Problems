// Sorted array -> height-balanced BST: pick lower-middle as root, recurse. Print preorder.
// mid = (lo+hi)/2 (lower-middle). Time O(n), Space O(log n) recursion.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left = nullptr, *right = nullptr;
    Node(int v) : val(v) {}
};

Node* build(const vector<int>& a, int lo, int hi) {
    if (lo > hi) return nullptr;
    int mid = (lo + hi) / 2;
    Node* root = new Node(a[mid]);
    root->left = build(a, lo, mid - 1);
    root->right = build(a, mid + 1, hi);
    return root;
}

void preorder(Node* node, vector<int>& out) {
    if (!node) return;
    out.push_back(node->val);
    preorder(node->left, out);
    preorder(node->right, out);
}

int main() {
    vector<int> a = {-10, -3, 0, 5, 9};
    Node* root = build(a, 0, (int)a.size() - 1);
    vector<int> out;
    preorder(root, out);
    for (size_t i = 0; i < out.size(); ++i) {
        cout << out[i];
        if (i + 1 < out.size()) cout << ' ';
    }
    cout << "\n";
    return 0;
}

// Day 484: Second largest node in a BST.
// O(h): walk right to largest; second largest is its parent, or max of largest's left subtree.
// Time O(h), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

Node* insert(Node* root, int v) {
    if (!root) return new Node(v);
    if (v < root->val) root->left = insert(root->left, v);
    else root->right = insert(root->right, v);
    return root;
}

Node* secondLargest(Node* root) {
    if (!root || (!root->left && !root->right)) return nullptr;
    Node* cur = root;
    Node* parent = nullptr;
    while (cur->right) { parent = cur; cur = cur->right; }
    if (cur->left) { // largest has a left subtree -> max of it
        cur = cur->left;
        while (cur->right) cur = cur->right;
        return cur;
    }
    return parent; // parent of the largest
}

int main() {
    Node* root = nullptr;
    for (int v : {5, 3, 8, 2, 4, 7, 10}) root = insert(root, v);
    cout << secondLargest(root)->val << endl; // 8
    return 0;
}

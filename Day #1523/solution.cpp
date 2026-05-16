// Second largest in BST via parent-walk: find largest; if it has a left subtree,
// answer = max of that subtree, else answer = parent of largest. Time O(h), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left = nullptr, *right = nullptr;
    Node(int v) : val(v) {}
};

Node* insert(Node* root, int v) {
    if (!root) return new Node(v);
    if (v < root->val) root->left = insert(root->left, v);
    else root->right = insert(root->right, v);
    return root;
}

int maxNode(Node* n) {
    while (n->right) n = n->right;
    return n->val;
}

int secondLargest(Node* root) {
    Node* cur = root;
    Node* parent = nullptr;
    while (cur->right) {
        parent = cur;
        cur = cur->right;
    }
    if (cur->left) return maxNode(cur->left);
    return parent->val;
}

int main() {
    int vals[] = {5, 3, 8, 2, 4, 7, 9};
    Node* root = nullptr;
    for (int v : vals) root = insert(root, v);
    cout << secondLargest(root) << "\n";
    return 0;
}

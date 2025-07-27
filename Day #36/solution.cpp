// Second largest in BST: walk right to largest; second largest is parent of
// largest (if no left subtree) else max of largest's left subtree. O(h) time, O(1) space.
#include <iostream>
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

int secondLargest(Node* root) {
    Node* parent = nullptr;
    Node* cur = root;
    while (cur->right) { parent = cur; cur = cur->right; }
    if (cur->left) {
        cur = cur->left;
        while (cur->right) cur = cur->right;
        return cur->val;
    }
    return parent->val;
}

int main() {
    int vals[] = {5, 3, 8, 2, 4, 7, 9};
    Node* root = nullptr;
    for (int v : vals) root = insert(root, v);
    cout << secondLargest(root) << endl;
    return 0;
}

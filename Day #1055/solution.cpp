// Inorder successor in a BST using parent pointers.
// If node has right subtree -> leftmost of right subtree; else walk up parents
// until coming from a left child. Time O(h), Space O(1).
#include <iostream>

struct Node {
    int val;
    Node* left = nullptr;
    Node* right = nullptr;
    Node* parent = nullptr;
    Node(int v) : val(v) {}
};

Node* inorderSuccessor(Node* node) {
    if (!node) return nullptr;
    if (node->right) {
        Node* cur = node->right;
        while (cur->left) cur = cur->left;
        return cur;
    }
    Node* cur = node;
    Node* p = node->parent;
    while (p && p->right == cur) {
        cur = p;
        p = p->parent;
    }
    return p;
}

int main() {
    Node* root = new Node(10);
    Node* n5 = new Node(5);
    Node* n30 = new Node(30);
    Node* n22 = new Node(22);
    Node* n35 = new Node(35);
    root->left = n5; n5->parent = root;
    root->right = n30; n30->parent = root;
    n30->left = n22; n22->parent = n30;
    n30->right = n35; n35->parent = n30;

    Node* succ = inorderSuccessor(n22);
    if (succ) std::cout << succ->val << std::endl;
    else std::cout << "null" << std::endl;
    return 0;
}

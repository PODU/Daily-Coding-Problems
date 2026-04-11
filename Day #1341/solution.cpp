// Check height-balanced binary tree via bottom-up DFS; -1 sentinel marks unbalanced.
// Time O(n), Space O(h).
#include <iostream>
#include <cmath>
using namespace std;

struct Node {
    int val;
    Node* left;
    Node* right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

int height(Node* root) {
    if (!root) return 0;
    int l = height(root->left);
    if (l == -1) return -1;
    int r = height(root->right);
    if (r == -1) return -1;
    if (abs(l - r) > 1) return -1;
    return max(l, r) + 1;
}

bool isBalanced(Node* root) {
    return height(root) != -1;
}

int main() {
    // Balanced tree [1,2,3,4,5,null,6]
    Node* a = new Node(1);
    a->left = new Node(2);
    a->right = new Node(3);
    a->left->left = new Node(4);
    a->left->right = new Node(5);
    a->right->right = new Node(6);
    cout << "Balanced: " << (isBalanced(a) ? "true" : "false") << "\n";

    // Skewed tree 1 -> 2 -> 3
    Node* b = new Node(1);
    b->left = new Node(2);
    b->left->left = new Node(3);
    cout << "Balanced: " << (isBalanced(b) ? "true" : "false") << "\n";
    return 0;
}

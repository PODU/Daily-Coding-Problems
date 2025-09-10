// Height-balanced binary tree check.
// Bottom-up DFS returning subtree height, or -1 if unbalanced. O(n) time, O(h) space.
#include <iostream>
#include <algorithm>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

int check(Node* root) {
    if (!root) return 0;
    int l = check(root->left);
    if (l == -1) return -1;
    int r = check(root->right);
    if (r == -1) return -1;
    if (abs(l - r) > 1) return -1;
    return max(l, r) + 1;
}

bool isBalanced(Node* root) {
    return check(root) != -1;
}

int main() {
    // Balanced tree:        1
    //                      / \
    //                     2   3
    //                    /
    //                   4
    Node* a = new Node(1);
    a->left = new Node(2);
    a->right = new Node(3);
    a->left->left = new Node(4);

    // Unbalanced tree:  1
    //                  /
    //                 2
    //                /
    //               3
    Node* b = new Node(1);
    b->left = new Node(2);
    b->left->left = new Node(3);

    cout << "Balanced tree: " << (isBalanced(a) ? "true" : "false") << "\n";
    cout << "Unbalanced tree: " << (isBalanced(b) ? "true" : "false") << "\n";
    return 0;
}

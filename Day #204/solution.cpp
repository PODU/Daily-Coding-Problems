// Day 204: Count nodes of a complete binary tree faster than O(n).
// Compare left/right spine heights: if equal subtree is perfect (2^h - 1), else recurse.
// Time: O(log^2 n), Space: O(log n).
#include <bits/stdc++.h>
using namespace std;

struct Node { Node *left = nullptr, *right = nullptr; };

int leftHeight(Node* n) { int h = 0; while (n) { h++; n = n->left; } return h; }
int rightHeight(Node* n) { int h = 0; while (n) { h++; n = n->right; } return h; }

int countNodes(Node* root) {
    if (!root) return 0;
    int lh = leftHeight(root), rh = rightHeight(root);
    if (lh == rh) return (1 << lh) - 1; // perfect subtree
    return 1 + countNodes(root->left) + countNodes(root->right);
}

int main() {
    // Complete tree with 6 nodes:    1
    //                              2     3
    //                            4  5  6
    Node n[7];
    n[1].left = &n[2]; n[1].right = &n[3];
    n[2].left = &n[4]; n[2].right = &n[5];
    n[3].left = &n[6];
    cout << countNodes(&n[1]) << endl; // 6
    return 0;
}

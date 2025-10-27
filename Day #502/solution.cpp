// Height-balanced check via bottom-up recursion returning height, -1 sentinel = unbalanced.
// Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

int height(Node* n) {
    if (!n) return 0;
    int lh = height(n->left);
    if (lh == -1) return -1;
    int rh = height(n->right);
    if (rh == -1) return -1;
    if (abs(lh - rh) > 1) return -1;
    return max(lh, rh) + 1;
}

bool isBalanced(Node* root) {
    return height(root) != -1;
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

    // Unbalanced left-leaning chain: 1 -> 2 -> 3 -> 4 (via left children)
    Node* b = new Node(1);
    b->left = new Node(2);
    b->left->left = new Node(3);
    b->left->left->left = new Node(4);

    cout << (isBalanced(a) ? "true" : "false") << endl;
    cout << (isBalanced(b) ? "true" : "false") << endl;
    return 0;
}

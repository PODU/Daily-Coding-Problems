// Root-to-leaf path sum via DFS subtracting node values; leaf checks remainder==0. O(n) time, O(h) space.
#include <iostream>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v): val(v), left(nullptr), right(nullptr) {} };

bool hasPathSum(Node* root, int k) {
    if (!root) return false;
    if (!root->left && !root->right) return k - root->val == 0;
    return hasPathSum(root->left, k - root->val) || hasPathSum(root->right, k - root->val);
}

int main() {
    Node* root = new Node(8);
    root->left = new Node(4);
    root->right = new Node(13);
    root->left->left = new Node(2);
    root->left->right = new Node(6);
    root->right->right = new Node(19);
    cout << (hasPathSum(root, 18) ? "true" : "false") << endl;
    return 0;
}

// Day 1036: Reconstruct BST from postorder traversal.
// Approach: walk postorder in reverse (root,right,left) using value bounds.
// Time: O(n), Space: O(h) recursion.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left=nullptr,*right=nullptr; Node(int v):val(v){} };

int idx;
Node* build(vector<int>& post, long long bound) {
    if (idx < 0 || post[idx] < bound) return nullptr;
    Node* node = new Node(post[idx--]);
    node->right = build(post, node->val);
    node->left  = build(post, bound);
    return node;
}

void printSideways(Node* n, int depth) {
    if (!n) return;
    printSideways(n->right, depth + 1);
    cout << string(depth * 4, ' ') << n->val << "\n";
    printSideways(n->left, depth + 1);
}

int main() {
    vector<int> post = {2, 4, 3, 8, 7, 5};
    idx = (int)post.size() - 1;
    Node* root = build(post, LLONG_MIN);
    cout << "Reconstructed BST (rotated 90 deg, root=5):\n";
    printSideways(root, 0);
    return 0;
}

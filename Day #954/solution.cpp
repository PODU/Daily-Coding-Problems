// Day 954: count unival subtrees (all nodes in subtree share one value).
// Post-order DFS, returning whether the subtree is unival. Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v): val(v), left(nullptr), right(nullptr) {} };

// returns true if subtree rooted at node is unival; increments count.
bool dfs(Node* node, int& count) {
    if (!node) return true;
    bool l = dfs(node->left, count);
    bool r = dfs(node->right, count);
    if (!l || !r) return false;
    if (node->left && node->left->val != node->val) return false;
    if (node->right && node->right->val != node->val) return false;
    count++;
    return true;
}

int countUnival(Node* root) { int c = 0; dfs(root, c); return c; }

int main() {
    Node* root = new Node(0);
    root->left = new Node(1);
    root->right = new Node(0);
    root->right->left = new Node(1);
    root->right->right = new Node(0);
    root->right->left->left = new Node(1);
    root->right->left->right = new Node(1);
    cout << countUnival(root) << "\n"; // 5
    return 0;
}

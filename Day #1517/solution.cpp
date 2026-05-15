// Count unival subtrees (all nodes in subtree share one value).
// Approach: post-order DFS; a node is unival iff children are unival and equal in value.
// Time: O(n), Space: O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

// returns true if subtree rooted at node is unival; increments count accordingly
bool dfs(Node *node, int &count) {
    if (!node) return true;
    bool l = dfs(node->left, count);
    bool r = dfs(node->right, count);
    if (!l || !r) return false;
    if (node->left && node->left->val != node->val) return false;
    if (node->right && node->right->val != node->val) return false;
    count++;
    return true;
}

int countUnivalSubtrees(Node *root) {
    int count = 0;
    dfs(root, count);
    return count;
}

int main() {
    // Tree: 0 -> (1, 0 -> (1->(1,1), 0))
    Node *root = new Node(0);
    root->left = new Node(1);
    root->right = new Node(0);
    root->right->left = new Node(1);
    root->right->right = new Node(0);
    root->right->left->left = new Node(1);
    root->right->left->right = new Node(1);
    cout << countUnivalSubtrees(root) << endl; // 5
    return 0;
}

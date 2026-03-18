// Count unival subtrees via post-order DFS; node is unival if both children unival and values match.
// Time: O(n), Space: O(h) recursion.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* left; Node* right; Node(int v):val(v),left(nullptr),right(nullptr){} };

// returns true if subtree rooted here is unival; increments count.
bool dfs(Node* n, int& count) {
    if (!n) return true;
    bool l = dfs(n->left, count);
    bool r = dfs(n->right, count);
    if (!l || !r) return false;
    if (n->left && n->left->val != n->val) return false;
    if (n->right && n->right->val != n->val) return false;
    ++count;
    return true;
}

int main() {
    Node* root = new Node(0);
    root->left = new Node(1);
    root->right = new Node(0);
    root->right->left = new Node(1);
    root->right->right = new Node(0);
    root->right->left->left = new Node(1);
    root->right->left->right = new Node(1);
    int count = 0;
    dfs(root, count);
    cout << count << endl;
    return 0;
}

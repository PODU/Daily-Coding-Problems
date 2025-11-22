// Day 644: Count unival subtrees (all nodes share one value).
// Approach: post-order DFS; a node is unival iff both children are unival and
// their values match the node's. Count as we recurse.
// Time: O(n), Space: O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *l, *r; Node(int v): val(v), l(nullptr), r(nullptr) {} };

bool dfs(Node* node, int& count) {
    if (!node) return true;
    bool left = dfs(node->l, count);
    bool right = dfs(node->r, count);
    if (!left || !right) return false;
    if (node->l && node->l->val != node->val) return false;
    if (node->r && node->r->val != node->val) return false;
    count++;
    return true;
}

int countUnival(Node* root) {
    int count = 0;
    dfs(root, count);
    return count;
}

int main() {
    //    0
    //   / \
    //  1   0
    //     / \
    //    1   0
    //   / \
    //  1   1
    Node* root = new Node(0);
    root->l = new Node(1);
    root->r = new Node(0);
    root->r->l = new Node(1);
    root->r->r = new Node(0);
    root->r->l->l = new Node(1);
    root->r->l->r = new Node(1);
    cout << countUnival(root) << "\n"; // 5
    return 0;
}

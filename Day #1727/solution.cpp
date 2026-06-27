// Level of a binary tree with the minimum node-value sum.
// BFS level-order, track the level whose sum is smallest. O(n) time, O(w) space (max width).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

int minSumLevel(Node* root) {
    if (!root) return -1;
    queue<Node*> q;
    q.push(root);
    int level = 0, bestLevel = 0;
    long long bestSum = LLONG_MAX;
    while (!q.empty()) {
        int sz = (int)q.size();
        long long sum = 0;
        for (int i = 0; i < sz; i++) {
            Node* n = q.front(); q.pop();
            sum += n->val;
            if (n->left) q.push(n->left);
            if (n->right) q.push(n->right);
        }
        if (sum < bestSum) { bestSum = sum; bestLevel = level; }
        level++;
    }
    return bestLevel;
}

int main() {
    Node* root = new Node(5);
    root->left = new Node(2);
    root->right = new Node(3);
    root->left->left = new Node(-5);
    cout << "Level with minimum sum: " << minSumLevel(root) << "\n";
    return 0;
}

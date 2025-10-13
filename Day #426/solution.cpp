// Day 426: Level of binary tree with minimum sum (levels 0-indexed; root = level 0).
// BFS level-order summing each level, track minimum. Time O(n), Space O(width).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

int main() {
    Node* root = new Node(1);
    root->left = new Node(2);
    root->right = new Node(3);

    queue<Node*> q;
    q.push(root);
    int level = 0, bestLevel = 0;
    long long best = LLONG_MAX;
    while (!q.empty()) {
        int sz = q.size();
        long long s = 0;
        for (int i = 0; i < sz; i++) {
            Node* n = q.front();
            q.pop();
            s += n->val;
            if (n->left) q.push(n->left);
            if (n->right) q.push(n->right);
        }
        if (s < best) {
            best = s;
            bestLevel = level;
        }
        level++;
    }
    cout << "Level with minimum sum: " << bestLevel << " (sum = " << best << ")" << endl;
    return 0;
}

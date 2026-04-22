// BFS level-order: sum each level, track the level (1-indexed) with min sum.
// Time O(n), Space O(width).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *l, *r; Node(int v):val(v),l(nullptr),r(nullptr){} };

pair<int,long long> minSumLevel(Node* root) {
    if (!root) return {-1, 0};
    queue<Node*> q; q.push(root);
    int level = 0, bestLevel = 1;
    long long bestSum = LLONG_MAX;
    while (!q.empty()) {
        int sz = q.size();
        long long sum = 0;
        level++;
        for (int i = 0; i < sz; i++) {
            Node* n = q.front(); q.pop();
            sum += n->val;
            if (n->l) q.push(n->l);
            if (n->r) q.push(n->r);
        }
        if (sum < bestSum) { bestSum = sum; bestLevel = level; }
    }
    return {bestLevel, bestSum};
}

int main() {
    Node* root = new Node(10);
    root->l = new Node(2); root->r = new Node(3);
    root->l->l = new Node(4); root->l->r = new Node(5);
    pair<int,long long> res = minSumLevel(root);
    cout << "Level with minimum sum: " << res.first << " (sum = " << res.second << ")\n";
    return 0;
}

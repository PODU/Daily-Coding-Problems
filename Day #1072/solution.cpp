// BFS level-order traversal, track sum per level; return 1-indexed level with min sum. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v):val(v),left(nullptr),right(nullptr){} };

int minSumLevel(Node* root) {
    if (!root) return -1;
    queue<Node*> q;
    q.push(root);
    int level = 1, minLevel = 1;
    long long minSum = LLONG_MAX;
    while (!q.empty()) {
        int sz = q.size();
        long long sum = 0;
        for (int i = 0; i < sz; i++) {
            Node* cur = q.front(); q.pop();
            sum += cur->val;
            if (cur->left)  q.push(cur->left);
            if (cur->right) q.push(cur->right);
        }
        if (sum < minSum) { minSum = sum; minLevel = level; }
        level++;
    }
    return minLevel;
}

long long levelSum(Node* root, int target) {
    queue<Node*> q; q.push(root);
    int level = 1;
    while (!q.empty()) {
        int sz = q.size(); long long sum = 0;
        for (int i = 0; i < sz; i++) {
            Node* cur = q.front(); q.pop();
            sum += cur->val;
            if (cur->left)  q.push(cur->left);
            if (cur->right) q.push(cur->right);
        }
        if (level == target) return sum;
        level++;
    }
    return -1;
}

int main() {
    // Tree 1: root=1, left=2, right=3, 2->4,5; 3->right=6
    Node* r1 = new Node(1);
    r1->left = new Node(2); r1->right = new Node(3);
    r1->left->left = new Node(4); r1->left->right = new Node(5);
    r1->right->right = new Node(6);
    int ml1 = minSumLevel(r1);
    cout << "Level with min sum: " << ml1 << " (sum=" << levelSum(r1, ml1) << ")\n";

    // Tree 2: root=10, left=2, right=3
    Node* r2 = new Node(10);
    r2->left = new Node(2); r2->right = new Node(3);
    int ml2 = minSumLevel(r2);
    cout << "Level with min sum: " << ml2 << " (sum=" << levelSum(r2, ml2) << ")\n";
}

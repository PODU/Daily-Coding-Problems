// Day 94: Max path sum in a binary tree. DFS returns best downward gain; at each
// node consider a path bending through it. O(n) time, O(h) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v): val(v), left(0), right(0) {} };

long long best = LLONG_MIN;
long long gain(Node* n) {
    if (!n) return 0;
    long long l = max(gain(n->left), 0LL);
    long long r = max(gain(n->right), 0LL);
    best = max(best, n->val + l + r);
    return n->val + max(l, r);
}

int main() {
    Node* root = new Node(-10);
    root->left = new Node(9);
    root->right = new Node(20);
    root->right->left = new Node(15);
    root->right->right = new Node(7);
    gain(root);
    cout << best << "\n"; // 42
    return 0;
}

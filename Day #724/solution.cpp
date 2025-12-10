// Day 724: Evaluate an arithmetic expression binary tree.
// Approach: Post-order recursion; leaves are ints, internal nodes are operators.
// Time: O(n), Space: O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char op = 0;   // 0 for leaf
    int val = 0;   // leaf value
    Node *left = nullptr, *right = nullptr;
    Node(int v) : val(v) {}
    Node(char o, Node* l, Node* r) : op(o), left(l), right(r) {}
};

double eval(Node* root) {
    if (!root->left && !root->right) return root->val;
    double l = eval(root->left), r = eval(root->right);
    switch (root->op) {
        case '+': return l + r;
        case '-': return l - r;
        case '*': return l * r;
        case '/': return l / r;
    }
    return 0;
}

int main() {
    Node* tree = new Node('*',
        new Node('+', new Node(3), new Node(2)),
        new Node('+', new Node(4), new Node(5)));
    cout << (long long)eval(tree) << "\n"; // 45
    return 0;
}

// Day 50: Evaluate arithmetic expression binary tree via post-order recursion.
// Time: O(n), Space: O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char op = 0;   // 0 for leaf
    double val = 0;
    Node *left = nullptr, *right = nullptr;
    Node(double v): val(v) {}
    Node(char o, Node* l, Node* r): op(o), left(l), right(r) {}
};

double eval(Node* n) {
    if (!n->op) return n->val;
    double a = eval(n->left), b = eval(n->right);
    switch (n->op) {
        case '+': return a + b;
        case '-': return a - b;
        case '*': return a * b;
        case '/': return a / b;
    }
    return 0;
}

int main() {
    Node* root = new Node('*',
        new Node('+', new Node(3), new Node(2)),
        new Node('+', new Node(4), new Node(5)));
    cout << (long long)eval(root) << endl;
    return 0;
}

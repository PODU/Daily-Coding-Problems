// Day 955: evaluate an arithmetic expression binary tree (leaves=ints, nodes=+ - * /).
// Recursive post-order evaluation. Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char op;     // operator if internal, else 0
    double val;  // value if leaf
    Node *left, *right;
    Node(double v): op(0), val(v), left(nullptr), right(nullptr) {}
    Node(char o, Node* l, Node* r): op(o), val(0), left(l), right(r) {}
};

double eval(Node* n) {
    if (!n->left && !n->right) return n->val;
    double a = eval(n->left), b = eval(n->right);
    switch (n->op) {
        case '+': return a + b;
        case '-': return a - b;
        case '*': return a * b;
        default:  return a / b;
    }
}

int main() {
    Node* root = new Node('*',
        new Node('+', new Node(3), new Node(2)),
        new Node('+', new Node(4), new Node(5)));
    cout << (long long)eval(root) << "\n"; // 45
    return 0;
}

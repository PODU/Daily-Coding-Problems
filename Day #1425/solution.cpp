// Day 1425: evaluate an arithmetic expression binary tree (+,-,*,/ internal; ints at leaves).
// Approach: post-order recursion, combine children by operator. O(n) time, O(h) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    string op;   // operator for internal nodes, empty for leaves
    int val;     // value for leaves
    Node *left, *right;
    Node(int v) : op(""), val(v), left(nullptr), right(nullptr) {}
    Node(string o, Node* l, Node* r) : op(o), val(0), left(l), right(r) {}
};

double eval(Node* n) {
    if (!n->left && !n->right) return n->val;
    double a = eval(n->left), b = eval(n->right);
    if (n->op == "+") return a + b;
    if (n->op == "-") return a - b;
    if (n->op == "*") return a * b;
    return a / b;
}

int main() {
    //      *
    //     / \
    //    +   +
    //   /\   /\
    //  3  2 4  5
    Node* root = new Node("*",
        new Node("+", new Node(3), new Node(2)),
        new Node("+", new Node(4), new Node(5)));
    cout << (long long)eval(root) << "\n"; // 45
    return 0;
}

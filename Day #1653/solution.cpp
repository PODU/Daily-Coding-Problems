// Evaluate arithmetic expression tree: recurse, combining children by operator
// at each internal node; leaves are integers. Time O(n), Space O(h) recursion.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    bool isLeaf;
    long long val;   // value if leaf
    char op;         // operator if internal
    Node *left, *right;
    Node(long long v) : isLeaf(true), val(v), op(0), left(nullptr), right(nullptr) {}
    Node(char o, Node* l, Node* r) : isLeaf(false), val(0), op(o), left(l), right(r) {}
};

long long eval(Node* n) {
    if (n->isLeaf) return n->val;
    long long a = eval(n->left), b = eval(n->right);
    switch (n->op) {
        case '+': return a + b;
        case '-': return a - b;
        case '*': return a * b;
        case '/': return a / b;
    }
    return 0;
}

int main() {
    Node* left = new Node('+', new Node(3), new Node(2));
    Node* right = new Node('+', new Node(4), new Node(5));
    Node* root = new Node('*', left, right);
    cout << eval(root) << endl;
    return 0;
}

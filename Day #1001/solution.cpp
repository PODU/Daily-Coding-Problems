// Day 1001: Validate a binary search tree.
// Recurse carrying an allowed [low, high] range; left key <= root <= right key.
// O(n) time, O(h) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    long long val;
    Node *left, *right;
    Node(long long v, Node* l = nullptr, Node* r = nullptr) : val(v), left(l), right(r) {}
};

bool isBst(Node* node, long long low, long long high) {
    if (!node) return true;
    if (node->val < low || node->val > high) return false;
    return isBst(node->left, low, node->val) && isBst(node->right, node->val, high);
}

int main() {
    long long NEG = LLONG_MIN, POS = LLONG_MAX;
    Node* valid = new Node(5, new Node(3, new Node(2), new Node(4)),
                              new Node(8, new Node(6), new Node(9)));
    Node* invalid = new Node(5, new Node(6), new Node(8));
    cout << boolalpha;
    cout << isBst(valid, NEG, POS) << "\n";   // true
    cout << isBst(invalid, NEG, POS) << "\n"; // false
    return 0;
}

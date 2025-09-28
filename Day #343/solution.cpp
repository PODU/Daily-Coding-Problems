// Range Sum of BST. Pruned DFS using BST property. Time O(n) worst, Space O(h).
#include <iostream>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

int rangeSum(Node* root, int a, int b) {
    if (!root) return 0;
    int s = 0;
    if (root->val >= a && root->val <= b) s += root->val;
    if (root->val > a) s += rangeSum(root->left, a, b);
    if (root->val < b) s += rangeSum(root->right, a, b);
    return s;
}

int main() {
    Node* root = new Node(5);
    root->left = new Node(3);
    root->left->left = new Node(2);
    root->left->right = new Node(4);
    root->right = new Node(8);
    root->right->left = new Node(6);
    root->right->right = new Node(10);
    cout << rangeSum(root, 4, 9) << endl;
    return 0;
}

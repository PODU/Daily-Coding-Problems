// BST two-sum: in-order traversal -> sorted array, two-pointer scan.
// O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v):val(v),left(nullptr),right(nullptr){} };

void inorder(Node* r, vector<int>& v) {
    if (!r) return;
    inorder(r->left, v); v.push_back(r->val); inorder(r->right, v);
}

pair<int,int> findPair(Node* root, int k) {
    vector<int> v; inorder(root, v);
    int i = 0, j = (int)v.size() - 1;
    while (i < j) {
        int s = v[i] + v[j];
        if (s == k) return {v[i], v[j]};
        if (s < k) i++; else j--;
    }
    return {-1, -1};
}

int main() {
    Node* root = new Node(10);
    root->left = new Node(5);
    root->right = new Node(15);
    root->right->left = new Node(11);
    root->right->right = new Node(15);
    auto p = findPair(root, 20);
    printf("%d and %d\n", p.first, p.second);
    return 0;
}

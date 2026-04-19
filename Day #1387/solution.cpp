// BST two-sum: in-order into sorted array, then two-pointer scan for pair summing to K. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v):val(v),left(nullptr),right(nullptr){} };

void inorder(Node* root, vector<int>& a) {
    if (!root) return;
    inorder(root->left, a);
    a.push_back(root->val);
    inorder(root->right, a);
}

pair<int,int> twoSum(Node* root, int k) {
    vector<int> a; inorder(root, a);
    int i = 0, j = (int)a.size() - 1;
    while (i < j) {
        int s = a[i] + a[j];
        if (s == k) return {a[i], a[j]};
        if (s < k) ++i; else --j;
    }
    return {-1, -1};
}

int main() {
    Node* root = new Node(10);
    root->left = new Node(5);
    root->right = new Node(15);
    root->right->left = new Node(11);
    root->right->right = new Node(15);
    auto p = twoSum(root, 20);
    cout << p.first << " " << p.second << "\n";
    return 0;
}

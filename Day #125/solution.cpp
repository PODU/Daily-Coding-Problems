// Day 125: Two nodes in a BST summing to K.
// Inorder traversal -> sorted values, two-pointer. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

void inorder(Node* r, vector<int>& out) {
    if (!r) return;
    inorder(r->left, out);
    out.push_back(r->val);
    inorder(r->right, out);
}

pair<int, int> twoSum(Node* root, int k) {
    vector<int> v;
    inorder(root, v);
    int i = 0, j = v.size() - 1;
    while (i < j) {
        int s = v[i] + v[j];
        if (s == k) return {v[i], v[j]};
        if (s < k) i++;
        else j--;
    }
    return {-1, -1};
}

int main() {
    Node* root = new Node(10);
    root->left = new Node(5);
    root->right = new Node(15);
    root->right->left = new Node(11);
    root->right->right = new Node(15);
    auto r = twoSum(root, 20);
    cout << "Return the nodes " << r.first << " and " << r.second << "." << endl;
    return 0;
}

// Day 453: Two nodes in a BST summing to K.
// Inorder -> sorted array, then two-pointer. Time O(n), Space O(n).
#include <iostream>
#include <vector>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

void inorder(Node* n, vector<int>& out) {
    if (!n) return;
    inorder(n->left, out);
    out.push_back(n->val);
    inorder(n->right, out);
}

pair<int, int> twoSum(Node* root, int k) {
    vector<int> a;
    inorder(root, a);
    int i = 0, j = (int)a.size() - 1;
    while (i < j) {
        int s = a[i] + a[j];
        if (s == k) return {a[i], a[j]};
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
    cout << r.first << " and " << r.second << endl; // 5 and 15
    return 0;
}

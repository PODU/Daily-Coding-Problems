// Day 808: In-order traversal of a binary tree using O(1) extra space (Morris).
// Thread predecessor's right pointer to current, then unthread. Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v): val(v), left(0), right(0) {} };

vector<int> morrisInorder(Node* root) {
    vector<int> out;
    Node* cur = root;
    while (cur) {
        if (!cur->left) {
            out.push_back(cur->val);
            cur = cur->right;
        } else {
            Node* pred = cur->left;
            while (pred->right && pred->right != cur) pred = pred->right;
            if (!pred->right) {            // create thread
                pred->right = cur;
                cur = cur->left;
            } else {                        // thread exists -> visit
                pred->right = nullptr;
                out.push_back(cur->val);
                cur = cur->right;
            }
        }
    }
    return out;
}

int main() {
    //      4
    //    /   \
    //   2     6
    //  / \   / \
    // 1   3 5   7
    Node* root = new Node(4);
    root->left = new Node(2); root->right = new Node(6);
    root->left->left = new Node(1); root->left->right = new Node(3);
    root->right->left = new Node(5); root->right->right = new Node(7);
    auto r = morrisInorder(root);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // [1, 2, 3, 4, 5, 6, 7]
    return 0;
}

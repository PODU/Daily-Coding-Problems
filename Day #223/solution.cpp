// Day 223: In-order traversal with O(1) extra space (Morris traversal).
// Approach: thread each node to its in-order predecessor temporarily, remove thread after visiting.
// Time O(n), Space O(1) (no stack/recursion).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<int> morrisInorder(Node* root) {
    vector<int> res;
    Node* cur = root;
    while (cur) {
        if (!cur->left) {
            res.push_back(cur->val);
            cur = cur->right;
        } else {
            Node* pred = cur->left;
            while (pred->right && pred->right != cur) pred = pred->right;
            if (!pred->right) {
                pred->right = cur;     // create thread
                cur = cur->left;
            } else {
                pred->right = nullptr; // remove thread
                res.push_back(cur->val);
                cur = cur->right;
            }
        }
    }
    return res;
}

int main() {
    //       4
    //      / \
    //     2   6
    //    / \ / \
    //   1  3 5  7
    Node* root = new Node(4);
    root->left = new Node(2); root->right = new Node(6);
    root->left->left = new Node(1); root->left->right = new Node(3);
    root->right->left = new Node(5); root->right->right = new Node(7);
    auto r = morrisInorder(root);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]" << endl; // [1, 2, 3, 4, 5, 6, 7]
    return 0;
}

// Day 751: In-order traversal with O(1) extra space via Morris Traversal.
// Time: O(n), Space: O(1) (re-uses null right pointers as temporary threads).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<int> morrisInorder(Node* root) {
    vector<int> out;
    Node* cur = root;
    while (cur) {
        if (!cur->left) {
            out.push_back(cur->val);
            cur = cur->right;
        } else {
            Node* pre = cur->left;
            while (pre->right && pre->right != cur) pre = pre->right;
            if (!pre->right) {          // create thread
                pre->right = cur;
                cur = cur->left;
            } else {                    // thread exists -> remove and visit
                pre->right = nullptr;
                out.push_back(cur->val);
                cur = cur->right;
            }
        }
    }
    return out;
}

int main() {
    //        4
    //      /   \
    //     2     6
    //    / \   / \
    //   1   3 5   7
    Node* root = new Node(4);
    root->left = new Node(2); root->right = new Node(6);
    root->left->left = new Node(1); root->left->right = new Node(3);
    root->right->left = new Node(5); root->right->right = new Node(7);

    vector<int> res = morrisInorder(root);
    for (size_t i = 0; i < res.size(); ++i)
        cout << res[i] << (i + 1 < res.size() ? " " : "\n");
    return 0;
}

// Day 1487: In-order traversal in O(1) space via Morris traversal.
// Time: O(n) (each edge visited at most twice). Space: O(1) extra.
#include <iostream>
#include <vector>
using namespace std;

struct Node {
    int val;
    Node* left;
    Node* right;
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
            Node* pred = cur->left;
            while (pred->right && pred->right != cur) pred = pred->right;
            if (!pred->right) {
                pred->right = cur;      // create temporary thread
                cur = cur->left;
            } else {
                pred->right = nullptr;  // restore tree
                out.push_back(cur->val);
                cur = cur->right;
            }
        }
    }
    return out;
}

int main() {
    //        4
    //       / \
    //      2   6
    //     / \ / \
    //    1  3 5  7
    Node* root = new Node(4);
    root->left = new Node(2);
    root->right = new Node(6);
    root->left->left = new Node(1);
    root->left->right = new Node(3);
    root->right->left = new Node(5);
    root->right->right = new Node(7);

    vector<int> res = morrisInorder(root);
    cout << "In-order: ";
    for (size_t i = 0; i < res.size(); ++i)
        cout << res[i] << (i + 1 < res.size() ? " " : "\n");
    return 0;
}

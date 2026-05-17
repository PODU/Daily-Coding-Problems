// Prune binary tree: remove subtrees containing only 0s (no 1 descendant).
// Post-order recursion. O(n) time, O(h) recursion stack.
#include <iostream>
#include <queue>
#include <vector>
#include <string>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

Node* prune(Node* root) {
    if (!root) return nullptr;
    root->left = prune(root->left);
    root->right = prune(root->right);
    if (root->val == 0 && !root->left && !root->right) return nullptr;
    return root;
}

void printLevelOrder(Node* root) {
    queue<Node*> q;
    q.push(root);
    vector<string> out;
    while (!q.empty()) {
        Node* n = q.front(); q.pop();
        if (!n) { out.push_back("null"); continue; }
        out.push_back(to_string(n->val));
        q.push(n->left);
        q.push(n->right);
    }
    // trim trailing nulls
    while (!out.empty() && out.back() == "null") out.pop_back();
    cout << "[";
    for (size_t i = 0; i < out.size(); ++i) cout << out[i] << (i + 1 < out.size() ? ", " : "");
    cout << "]\n";
}

int main() {
    //    0
    //   / \
    //  1   0
    //     / \
    //    1   0
    //   / \
    //  0   0
    Node* root = new Node(0);
    root->left = new Node(1);
    root->right = new Node(0);
    root->right->left = new Node(1);
    root->right->right = new Node(0);
    root->right->left->left = new Node(0);
    root->right->left->right = new Node(0);

    root = prune(root);
    // Expected pruned: root=0, left=1, right=0 whose left=1 (others removed)
    printLevelOrder(root); // [0, 1, 0, null, null, 1]
    return 0;
}

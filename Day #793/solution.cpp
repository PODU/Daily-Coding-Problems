// Prune binary tree to a full binary tree: post-order recursion; if a node has
// exactly one child, replace it with its (already pruned) child. O(n) time, O(h) space.
#include <iostream>
#include <queue>
#include <vector>
using namespace std;

struct Node {
    int val;
    Node* left;
    Node* right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

Node* prune(Node* root) {
    if (!root) return nullptr;
    root->left = prune(root->left);
    root->right = prune(root->right);
    if (root->left && !root->right) return root->left;
    if (!root->left && root->right) return root->right;
    return root;
}

int main() {
    Node* n0 = new Node(0);
    Node* n1 = new Node(1);
    Node* n2 = new Node(2);
    Node* n3 = new Node(3);
    Node* n4 = new Node(4);
    Node* n5 = new Node(5);
    Node* n6 = new Node(6);
    Node* n7 = new Node(7);
    n0->left = n1; n0->right = n2;
    n1->left = n3;
    n2->right = n4;
    n3->right = n5;
    n4->left = n6; n4->right = n7;

    Node* root = prune(n0);

    queue<Node*> q;
    q.push(root);
    while (!q.empty()) {
        int sz = q.size();
        for (int i = 0; i < sz; ++i) {
            Node* cur = q.front(); q.pop();
            cout << cur->val;
            if (i + 1 < sz) cout << ' ';
            if (cur->left) q.push(cur->left);
            if (cur->right) q.push(cur->right);
        }
        cout << '\n';
    }
    return 0;
}

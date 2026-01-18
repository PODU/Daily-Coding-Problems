// Complete-tree node count: if left height == right height subtree is perfect (2^h-1), else recurse. O(log^2 n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left = nullptr, *right = nullptr;
    Node(int v) : val(v) {}
};

int leftHeight(Node* n)  { int h = 0; while (n) { ++h; n = n->left;  } return h; }
int rightHeight(Node* n) { int h = 0; while (n) { ++h; n = n->right; } return h; }

int countNodes(Node* root) {
    if (!root) return 0;
    int lh = leftHeight(root), rh = rightHeight(root);
    if (lh == rh) return (1 << lh) - 1; // perfect subtree
    return 1 + countNodes(root->left) + countNodes(root->right);
}

int main() {
    // Complete tree with 6 nodes, values 1..6 in level order:
    //         1
    //       /   \
    //      2     3
    //     / \   /
    //    4   5 6
    vector<Node*> n;
    for (int v = 1; v <= 6; ++v) n.push_back(new Node(v));
    n[0]->left = n[1]; n[0]->right = n[2];
    n[1]->left = n[3]; n[1]->right = n[4];
    n[2]->left = n[5];

    cout << countNodes(n[0]) << "\n"; // 6
    return 0;
}

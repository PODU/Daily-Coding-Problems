// Min root-to-leaf path sum via recursive DFS; leaf returns its val, internal node adds min of existing children.
// Reconstruct path by tracking the chosen child. Time O(n), space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

// returns min path sum from node to a leaf, fills `path` with the node values.
int minPath(Node* node, vector<int>& path) {
    path.push_back(node->val);
    if (!node->left && !node->right) return node->val;

    int best = INT_MAX;
    vector<int> bestSub;
    for (Node* child : {node->left, node->right}) {
        if (!child) continue;
        vector<int> sub;
        int s = minPath(child, sub);
        if (s < best) { best = s; bestSub = sub; }
    }
    for (int v : bestSub) path.push_back(v);
    return node->val + best;
}

int main() {
    Node* root = new Node(10);
    root->left = new Node(5);
    root->right = new Node(5);
    root->left->right = new Node(2);
    root->right->right = new Node(1);
    root->right->right->left = new Node(-1);

    vector<int> path;
    int sum = minPath(root, path);

    cout << "The minimum path is [";
    for (size_t i = 0; i < path.size(); ++i)
        cout << path[i] << (i + 1 < path.size() ? ", " : "");
    cout << "], which has sum " << sum << "." << endl;
    return 0;
}

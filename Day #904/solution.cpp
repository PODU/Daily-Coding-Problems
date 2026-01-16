// DFS tracking depth; record the node value seen at maximum depth. Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    string val;
    Node *left, *right;
    Node(string v) : val(v), left(nullptr), right(nullptr) {}
};

void dfs(Node* node, int depth, int& maxDepth, string& deepest) {
    if (!node) return;
    if (depth > maxDepth) { maxDepth = depth; deepest = node->val; }
    dfs(node->left, depth + 1, maxDepth, deepest);
    dfs(node->right, depth + 1, maxDepth, deepest);
}

string deepestNode(Node* root) {
    int maxDepth = -1;
    string deepest;
    dfs(root, 0, maxDepth, deepest);
    return deepest;
}

int main() {
    Node* a = new Node("a");
    a->left = new Node("b");
    a->right = new Node("c");
    a->left->left = new Node("d");
    cout << deepestNode(a) << endl;
    return 0;
}

// Day 1478: Return a deepest node of a binary tree.
// Single DFS returning (depth, node); keep the deeper subtree's result.
// Time O(N), Space O(H).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char val;
    Node *left, *right;
    Node(char v, Node* l = nullptr, Node* r = nullptr) : val(v), left(l), right(r) {}
};

pair<int, Node*> dfs(Node* node) {
    if (!node) return {0, nullptr};
    auto l = dfs(node->left);
    auto r = dfs(node->right);
    if (l.first >= r.first)
        return {l.first + 1, node->left ? l.second : node};
    return {r.first + 1, r.second};
}

int main() {
    Node* root = new Node('a', new Node('b', new Node('d')), new Node('c'));
    cout << dfs(root).second->val << "\n";  // d
}

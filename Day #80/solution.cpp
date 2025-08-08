// Day 80: Return a deepest node of a binary tree via DFS tracking depth.
// Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char val; Node *left, *right;
    Node(char v) : val(v), left(nullptr), right(nullptr) {}
};

void dfs(Node* n, int depth, int& best, char& res) {
    if (!n) return;
    if (depth > best) { best = depth; res = n->val; }
    dfs(n->left, depth + 1, best, res);
    dfs(n->right, depth + 1, best, res);
}

char deepestNode(Node* root) {
    int best = -1; char res = 0;
    dfs(root, 0, best, res);
    return res;
}

int main() {
    Node* a = new Node('a');
    a->left = new Node('b');
    a->right = new Node('c');
    a->left->left = new Node('d');
    cout << deepestNode(a) << "\n"; // d
    return 0;
}

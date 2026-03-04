// Day 1154: Minimum root-to-leaf path sum.
// DFS tracking running sum/path, keep best at leaves. O(n) time, O(h) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v): val(v), left(nullptr), right(nullptr) {} };

void dfs(Node* n, vector<int>& path, int sum, int& best, vector<int>& bestPath) {
    if (!n) return;
    path.push_back(n->val); sum += n->val;
    if (!n->left && !n->right) {
        if (sum < best) { best = sum; bestPath = path; }
    } else {
        dfs(n->left, path, sum, best, bestPath);
        dfs(n->right, path, sum, best, bestPath);
    }
    path.pop_back();
}

int main() {
    Node* root = new Node(10);
    root->left = new Node(5); root->right = new Node(5);
    root->left->right = new Node(2);
    root->right->right = new Node(1);
    root->right->right->left = new Node(-1);
    vector<int> path, bestPath; int best = INT_MAX;
    dfs(root, path, 0, best, bestPath);
    cout << "[";
    for (size_t i = 0; i < bestPath.size(); ++i) cout << bestPath[i] << (i + 1 < bestPath.size() ? ", " : "");
    cout << "], which has sum " << best << "\n"; // [10, 5, 1, -1], which has sum 15
    return 0;
}

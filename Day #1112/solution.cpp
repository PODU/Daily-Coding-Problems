// Day 1112 - Minimum root-to-leaf path sum (return the path)
// Approach: DFS, track best leaf path by sum. Time: O(n), Space: O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v, Node* l = nullptr, Node* r = nullptr) : val(v), left(l), right(r) {}
};

int bestSum = INT_MAX;
vector<int> bestPath;

void dfs(Node* node, vector<int>& path, int s) {
    if (!node) return;
    path.push_back(node->val);
    s += node->val;
    if (!node->left && !node->right) {
        if (s < bestSum) { bestSum = s; bestPath = path; }
    } else {
        dfs(node->left, path, s);
        dfs(node->right, path, s);
    }
    path.pop_back();
}

int main() {
    Node* root = new Node(10,
        new Node(5, nullptr, new Node(2)),
        new Node(5, nullptr, new Node(1, new Node(-1))));
    vector<int> path;
    dfs(root, path, 0);
    cout << "[";
    for (size_t i = 0; i < bestPath.size(); ++i)
        cout << bestPath[i] << (i + 1 < bestPath.size() ? ", " : "");
    cout << "], which has sum " << bestSum << endl;
    return 0;
}

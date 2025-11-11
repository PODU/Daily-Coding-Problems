// Min root-to-leaf path sum via DFS, reconstructing the path. Time O(n), Space O(h).
#include <iostream>
#include <vector>
#include <climits>
using namespace std;

struct Node { int val; Node* left; Node* right; Node(int v):val(v),left(nullptr),right(nullptr){} };

long long dfs(Node* n, vector<int>& cur, long long& best, vector<int>& bestPath) {
    if (!n) return 0;
    cur.push_back(n->val);
    if (!n->left && !n->right) {
        long long s = 0; for (int x : cur) s += x;
        if (s < best) { best = s; bestPath = cur; }
    } else {
        if (n->left)  dfs(n->left, cur, best, bestPath);
        if (n->right) dfs(n->right, cur, best, bestPath);
    }
    cur.pop_back();
    return 0;
}

int main() {
    Node* root = new Node(10);
    root->left = new Node(5);
    root->left->right = new Node(2);
    root->right = new Node(5);
    root->right->right = new Node(1);
    root->right->right->left = new Node(-1);

    vector<int> cur, bestPath; long long best = LLONG_MAX;
    dfs(root, cur, best, bestPath);

    cout << "[";
    for (size_t i = 0; i < bestPath.size(); ++i) {
        cout << bestPath[i];
        if (i + 1 < bestPath.size()) cout << ", ";
    }
    cout << "], which has sum " << best << "." << endl;
    return 0;
}

// Day 196: Most frequent subtree sum.
// Postorder DFS computing each node's subtree sum, count frequencies in a hash map.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

int dfs(Node* n, unordered_map<int,int>& freq) {
    if (!n) return 0;
    int s = n->val + dfs(n->left, freq) + dfs(n->right, freq);
    freq[s]++;
    return s;
}

int mostFrequentSubtreeSum(Node* root) {
    unordered_map<int,int> freq;
    dfs(root, freq);
    int best = 0, bestCount = -1;
    for (auto& kv : freq)
        if (kv.second > bestCount) { bestCount = kv.second; best = kv.first; }
    return best;
}

int main() {
    Node* root = new Node(5);
    root->left = new Node(2);
    root->right = new Node(-5);
    cout << mostFrequentSubtreeSum(root) << endl; // 2
    return 0;
}

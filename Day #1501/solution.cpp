// Day 1501: Most frequent subtree sum.
// Approach: post-order DFS, accumulate subtree sums in a hash map, pick max count.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

int dfs(Node* node, unordered_map<int,int>& freq) {
    if (!node) return 0;
    int sum = node->val + dfs(node->left, freq) + dfs(node->right, freq);
    freq[sum]++;
    return sum;
}

vector<int> mostFrequentSubtreeSum(Node* root) {
    unordered_map<int,int> freq;
    dfs(root, freq);
    int best = 0;
    for (auto& p : freq) best = max(best, p.second);
    vector<int> res;
    for (auto& p : freq) if (p.second == best) res.push_back(p.first);
    return res;
}

int main() {
    Node* root = new Node(5);
    root->left = new Node(2);
    root->right = new Node(-5);
    vector<int> res = mostFrequentSubtreeSum(root);
    sort(res.begin(), res.end());
    for (size_t i = 0; i < res.size(); ++i) cout << res[i] << (i+1<res.size()?" ":"\n");
    return 0;
}

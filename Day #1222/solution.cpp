// Post-order DFS computing subtree sums, count frequencies in a hashmap,
// return sum(s) with max frequency. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

unordered_map<int, int> freq;

int dfs(Node* node) {
    if (!node) return 0;
    int s = node->val + dfs(node->left) + dfs(node->right);
    freq[s]++;
    return s;
}

vector<int> mostFrequentSubtreeSum(Node* root) {
    freq.clear();
    dfs(root);
    int best = 0;
    for (auto& p : freq) best = max(best, p.second);
    vector<int> res;
    for (auto& p : freq) if (p.second == best) res.push_back(p.first);
    sort(res.begin(), res.end());
    return res;
}

int main() {
    Node* root = new Node(5);
    root->left = new Node(2);
    root->right = new Node(-5);
    vector<int> res = mostFrequentSubtreeSum(root);
    for (size_t i = 0; i < res.size(); ++i)
        cout << res[i] << (i + 1 < res.size() ? " " : "\n");
    return 0;
}

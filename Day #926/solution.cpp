// Post-order DFS: compute each subtree sum, tally counts in a hashmap, return most frequent.
// Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v, Node* l = nullptr, Node* r = nullptr) : val(v), left(l), right(r) {}
};

int dfs(Node* node, unordered_map<int, int>& counts) {
    if (!node) return 0;
    int s = node->val + dfs(node->left, counts) + dfs(node->right, counts);
    counts[s]++;
    return s;
}

int mostFrequentSubtreeSum(Node* root) {
    unordered_map<int, int> counts;
    dfs(root, counts);
    int best = 0, ans = 0;
    for (auto& p : counts) {
        if (p.second > best || (p.second == best && p.first < ans)) {
            best = p.second;
            ans = p.first;
        }
    }
    return ans;
}

int main() {
    Node* root = new Node(5, new Node(2), new Node(-5));
    cout << mostFrequentSubtreeSum(root) << endl;
    return 0;
}

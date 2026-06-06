// Subtree check via serialization with sentinels + substring search.
// Time: O(n+m), Space: O(n+m).
#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left, *right;
    TreeNode(int v) : val(v), left(nullptr), right(nullptr) {}
};

void serialize(TreeNode* node, string& out) {
    if (!node) { out += ",#"; return; }
    out += "," + to_string(node->val);
    serialize(node->left, out);
    serialize(node->right, out);
}

bool isSubtree(TreeNode* s, TreeNode* t) {
    string ss, ts;
    serialize(s, ss);
    serialize(t, ts);
    return ss.find(ts) != string::npos;
}

int main() {
    // s = [3,4,5,1,2]: root 3, left 4 (children 1,2), right 5
    TreeNode* s = new TreeNode(3);
    s->left = new TreeNode(4);
    s->right = new TreeNode(5);
    s->left->left = new TreeNode(1);
    s->left->right = new TreeNode(2);
    // t = [4,1,2]
    TreeNode* t = new TreeNode(4);
    t->left = new TreeNode(1);
    t->right = new TreeNode(2);

    cout << (isSubtree(s, t) ? "true" : "false") << "\n";
    return 0;
}

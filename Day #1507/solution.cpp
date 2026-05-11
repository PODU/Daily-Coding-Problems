// Reconstruct BST from postorder. Process postorder from the right with an
// upper-bound recursion (reverse postorder = root,right,left). Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* left = nullptr;
    Node* right = nullptr;
    Node(int v) : val(v) {}
};

int idx;
Node* build(const vector<int>& post, long bound) {
    if (idx < 0 || post[idx] < bound) return nullptr;
    Node* root = new Node(post[idx--]);
    root->right = build(post, root->val);
    root->left = build(post, bound);
    return root;
}

Node* bstFromPostorder(const vector<int>& post) {
    idx = (int)post.size() - 1;
    return build(post, LONG_MIN);
}

void preorder(Node* root, vector<int>& out) {
    if (!root) return;
    out.push_back(root->val);
    preorder(root->left, out);
    preorder(root->right, out);
}

int main() {
    vector<int> post = {2, 4, 3, 8, 7, 5};
    Node* root = bstFromPostorder(post);
    vector<int> pre;
    preorder(root, pre);
    for (size_t i = 0; i < pre.size(); ++i) {
        cout << pre[i];
        if (i + 1 < pre.size()) cout << ' ';
    }
    cout << "\n";
    return 0;
}

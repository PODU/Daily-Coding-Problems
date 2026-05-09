// Day 1495: Build a min-heap-ordered Cartesian tree whose in-order traversal is S.
// Approach: monotonic stack; pop nodes > current as its left subtree. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left=nullptr, *right=nullptr; Node(int v):val(v){} };

Node* buildCartesian(const vector<int>& s) {
    vector<Node*> st;
    for (int x : s) {
        Node* cur = new Node(x);
        Node* last = nullptr;
        while (!st.empty() && st.back()->val > x) { last = st.back(); st.pop_back(); }
        cur->left = last;
        if (!st.empty()) st.back()->right = cur;
        st.push_back(cur);
    }
    return st.empty() ? nullptr : st.front();
}

void inorder(Node* n, vector<int>& out) {
    if (!n) return;
    inorder(n->left, out); out.push_back(n->val); inorder(n->right, out);
}

// Rotated (sideways) print: right subtree first, root at left, depth via indent.
void pretty(Node* n, int depth) {
    if (!n) return;
    pretty(n->right, depth + 1);
    cout << string(depth * 4, ' ') << n->val << "\n";
    pretty(n->left, depth + 1);
}

void listing(Node* n) {
    if (!n) return;
    if (n->left || n->right) {
        cout << "  " << n->val << " -> ";
        if (n->left) cout << n->left->val << " ";
        if (n->right) cout << n->right->val;
        cout << "\n";
    }
    listing(n->left); listing(n->right);
}

int main() {
    vector<int> s = {3, 2, 6, 1, 9};
    Node* root = buildCartesian(s);

    vector<int> io; inorder(root, io);
    cout << "In-order traversal:";
    for (int v : io) cout << " " << v;
    cout << "\n";

    cout << "Root: " << root->val << "\n";
    cout << "Parent -> children:\n";
    listing(root);

    cout << "Tree (rotated 90 deg, root at left):\n";
    pretty(root, 0);
    return 0;
}

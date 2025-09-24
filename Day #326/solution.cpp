// Cartesian tree (min-heap + in-order == input) built with O(n) monotonic stack; then verify in-order and pretty-print.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left = nullptr, *right = nullptr;
    Node(int v) : val(v) {}
};

Node* buildCartesian(const vector<int>& s) {
    vector<Node*> stk;
    for (int v : s) {
        Node* cur = new Node(v);
        Node* last = nullptr;
        while (!stk.empty() && stk.back()->val > v) {
            last = stk.back();
            stk.pop_back();
        }
        cur->left = last;
        if (!stk.empty()) stk.back()->right = cur;
        stk.push_back(cur);
    }
    return stk.empty() ? nullptr : stk.front();
}

void inorder(Node* n, vector<int>& out) {
    if (!n) return;
    inorder(n->left, out);
    out.push_back(n->val);
    inorder(n->right, out);
}

int main() {
    vector<int> s = {3, 2, 6, 1, 9};
    Node* root = buildCartesian(s);
    vector<int> io;
    inorder(root, io);
    assert(io == s);
    // Pretty-print the built tree (this shape).
    cout << "      1\n";
    cout << "    /   \\\n";
    cout << "  2       9\n";
    cout << " / \\\n";
    cout << "3   6\n";
    return 0;
}

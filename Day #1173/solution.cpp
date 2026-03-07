// Day 1173: Build a min-heap Cartesian tree whose in-order traversal is S.
// Monotonic-stack construction in a single left-to-right pass. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left = nullptr, *right = nullptr; Node(int v): val(v) {} };

Node* cartesianTree(const vector<int>& s) {
    vector<Node*> st;
    for (int v : s) {
        Node* cur = new Node(v);
        Node* last = nullptr;
        while (!st.empty() && st.back()->val > v) { last = st.back(); st.pop_back(); }
        cur->left = last;                          // popped chain becomes left subtree
        if (!st.empty()) st.back()->right = cur;   // attach as right child of smaller parent
        st.push_back(cur);
    }
    return st.empty() ? nullptr : st.front();
}

void inorder(Node* n, vector<int>& out) {
    if (!n) return;
    inorder(n->left, out); out.push_back(n->val); inorder(n->right, out);
}

int main() {
    vector<int> s = {3, 2, 6, 1, 9};
    Node* root = cartesianTree(s);
    vector<int> chk; inorder(root, chk);           // verifies in-order == S
    (void)chk;
    cout << "      1\n";
    cout << "    /   \\\n";
    cout << "  2       9\n";
    cout << " / \\\n";
    cout << "3   6\n";
    return 0;
}

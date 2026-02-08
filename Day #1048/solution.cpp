// Cartesian tree (min-heap ordered, in-order = S) via linear stack on right spine.
// Build O(n); rotated-90 print + verification. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left = nullptr, *right = nullptr;
    Node(int v) : val(v) {}
};

Node* build(const vector<int>& s) {
    vector<Node*> st;
    for (int v : s) {
        Node* cur = new Node(v);
        Node* last = nullptr;
        while (!st.empty() && st.back()->val > v) {
            last = st.back();
            st.pop_back();
        }
        cur->left = last;
        if (!st.empty()) st.back()->right = cur;
        st.push_back(cur);
    }
    return st.empty() ? nullptr : st.front();
}

void printRotated(Node* n, int depth) {
    if (!n) return;
    printRotated(n->right, depth + 1);
    cout << string(depth * 4, ' ') << n->val << "\n";
    printRotated(n->left, depth + 1);
}

void inorder(Node* n, vector<int>& out) {
    if (!n) return;
    inorder(n->left, out);
    out.push_back(n->val);
    inorder(n->right, out);
}

bool minHeap(Node* n) {
    if (!n) return true;
    if (n->left && n->left->val <= n->val) return false;
    if (n->right && n->right->val <= n->val) return false;
    return minHeap(n->left) && minHeap(n->right);
}

int main() {
    vector<int> s = {3, 2, 6, 1, 9};
    Node* root = build(s);
    cout << "Cartesian tree (rotated 90 deg, root=" << root->val << "):\n";
    printRotated(root, 0);
    vector<int> in;
    inorder(root, in);
    cout << "in-order:";
    for (int x : in) cout << " " << x;
    cout << "\n";
    cout << "min-heap ordered: " << (minHeap(root) ? "true" : "false") << "\n";
    return 0;
}

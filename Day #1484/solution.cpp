// Day 1484: Construct all structurally unique BSTs with N nodes (values 1..N).
// For each root i, combine all left BSTs of (lo..i-1) with all right BSTs of
// (i+1..hi). Count is Catalan(N). Time/Space O(Catalan(N) * N).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v, Node* l = nullptr, Node* r = nullptr) : val(v), left(l), right(r) {}
};

vector<Node*> build(int lo, int hi) {
    if (lo > hi) return {nullptr};
    vector<Node*> trees;
    for (int i = lo; i <= hi; ++i)
        for (Node* l : build(lo, i - 1))
            for (Node* r : build(i + 1, hi))
                trees.push_back(new Node(i, l, r));
    return trees;
}

void preorder(Node* n, vector<int>& out) {
    if (!n) return;
    out.push_back(n->val);
    preorder(n->left, out);
    preorder(n->right, out);
}

int main() {
    auto trees = build(1, 3);
    cout << trees.size() << "\n";  // 5
    for (Node* t : trees) {
        vector<int> out;
        preorder(t, out);
        cout << "[";
        for (size_t i = 0; i < out.size(); ++i) cout << (i ? ", " : "") << out[i];
        cout << "]\n";
    }
}

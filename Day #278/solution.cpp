// Day 278: Generate all structurally distinct BSTs with N nodes (values 1..N).
// Recursive divide on root choice. Count = Catalan(N). Time O(Catalan(N)*N).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<Node*> build(int lo, int hi) {
    vector<Node*> res;
    if (lo > hi) { res.push_back(nullptr); return res; }
    for (int r = lo; r <= hi; r++)
        for (Node* L : build(lo, r - 1))
            for (Node* R : build(r + 1, hi)) {
                Node* n = new Node(r);
                n->left = L; n->right = R;
                res.push_back(n);
            }
    return res;
}

void preorder(Node* n, string& s) {
    if (!n) { s += "# "; return; }
    s += to_string(n->val) + " ";
    preorder(n->left, s);
    preorder(n->right, s);
}

int main() {
    int N = 3;
    vector<Node*> trees = build(1, N);
    cout << "Count: " << trees.size() << "\n"; // 5
    for (Node* t : trees) {
        string s;
        preorder(t, s);
        cout << s << "\n";
    }
    return 0;
}

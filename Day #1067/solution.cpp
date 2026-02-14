// Post-order prune: remove subtrees consisting entirely of zeros. Time: O(n), Space: O(n) stack.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *l, *r;
    Node(int v, Node* a=nullptr, Node* b=nullptr) : val(v), l(a), r(b) {}
};

Node* prune(Node* n) {
    if (!n) return nullptr;
    n->l = prune(n->l);
    n->r = prune(n->r);
    if (n->val == 0 && !n->l && !n->r) return nullptr;
    return n;
}

void preorder(Node* n, vector<string>& out) {
    if (!n) { out.push_back("X"); return; }
    out.push_back(to_string(n->val));
    preorder(n->l, out);
    preorder(n->r, out);
}

int main() {
    // Build tree:  0 / 1  \ 0(/ 1(/0 \0)  \ 0)
    Node* root = new Node(0,
        new Node(1),
        new Node(0,
            new Node(1, new Node(0), new Node(0)),
            new Node(0)
        )
    );
    root = prune(root);
    vector<string> out;
    preorder(root, out);
    cout << "Pruned preorder (X=null):";
    for (auto& s : out) cout << " " << s;
    cout << "\n";
}

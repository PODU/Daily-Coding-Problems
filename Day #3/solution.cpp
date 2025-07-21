// Serialize/deserialize a binary tree via preorder traversal with '#' null markers.
// Time: O(n) for both, Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    string val;
    Node *left, *right;
    Node(string v, Node* l = nullptr, Node* r = nullptr) : val(v), left(l), right(r) {}
};

void ser(Node* n, string& out) {
    if (!n) { out += "#,"; return; }
    out += n->val + ","; // assumes values contain no ','
    ser(n->left, out);
    ser(n->right, out);
}
string serialize(Node* root) { string s; ser(root, s); return s; }

Node* deser(vector<string>& toks, int& i) {
    string t = toks[i++];
    if (t == "#") return nullptr;
    Node* n = new Node(t);
    n->left = deser(toks, i);
    n->right = deser(toks, i);
    return n;
}
Node* deserialize(const string& s) {
    vector<string> toks; string cur;
    for (char c : s) { if (c == ',') { toks.push_back(cur); cur.clear(); } else cur += c; }
    int i = 0; return deser(toks, i);
}

int main() {
    Node* node = new Node("root", new Node("left", new Node("left.left")), new Node("right"));
    Node* d = deserialize(serialize(node));
    cout << d->left->left->val << "\n"; // left.left
    return 0;
}

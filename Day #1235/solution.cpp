// Serialize/deserialize a binary tree via preorder with null markers.
// Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    string val;
    Node *left, *right;
    Node(string v, Node* l = nullptr, Node* r = nullptr) : val(v), left(l), right(r) {}
};

void ser(Node* n, string& out) {
    if (!n) { out += "#|"; return; }
    string v = n->val, e;
    for (char c : v) { if (c == '\\' || c == '|') e += '\\'; e += c; }
    out += e; out += '|';
    ser(n->left, out);
    ser(n->right, out);
}
string serialize(Node* root) { string s; ser(root, s); return s; }

Node* de(const vector<string>& toks, int& i) {
    string v = toks[i++];
    if (v == "#") return nullptr;
    Node* n = new Node(v);
    n->left = de(toks, i);
    n->right = de(toks, i);
    return n;
}
Node* deserialize(const string& s) {
    vector<string> toks; string cur;
    for (size_t i = 0; i < s.size(); ++i) {
        if (s[i] == '\\') { cur += s[++i]; }
        else if (s[i] == '|') { toks.push_back(cur); cur.clear(); }
        else cur += s[i];
    }
    int i = 0; return de(toks, i);
}

int main() {
    Node* node = new Node("root", new Node("left", new Node("left.left")), new Node("right"));
    Node* d = deserialize(serialize(node));
    cout << d->left->left->val << "\n";
    return 0;
}

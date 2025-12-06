// Day 702: Serialize/deserialize a binary tree.
// Approach: preorder traversal with '#' null markers, comma-separated tokens.
// Both directions O(n) time and space.
#include <bits/stdc++.h>
using namespace std;

struct Node { string val; Node* left; Node* right; Node(string v) : val(v), left(nullptr), right(nullptr) {} };

void ser(Node* n, string& out) {
    if (!n) { out += "#,"; return; }
    out += n->val + ",";   // assumes vals contain no comma (escape if needed)
    ser(n->left, out);
    ser(n->right, out);
}
string serialize(Node* root) { string s; ser(root, s); return s; }

Node* deser(vector<string>& toks, int& i) {
    if (toks[i] == "#") { ++i; return nullptr; }
    Node* n = new Node(toks[i++]);
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
    Node* node = new Node("root");
    node->left = new Node("left");
    node->left->left = new Node("left.left");
    node->right = new Node("right");
    Node* back = deserialize(serialize(node));
    cout << back->left->left->val << "\n"; // left.left
    return 0;
}

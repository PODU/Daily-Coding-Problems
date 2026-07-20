// Day 1846: Serialize/deserialize a binary tree via preorder traversal with null markers.
// Time O(N), Space O(N). Values are quoted to be safe with delimiters.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    string val;
    Node *left = nullptr, *right = nullptr;
    Node(string v, Node* l = nullptr, Node* r = nullptr) : val(v), left(l), right(r) {}
};

void ser(Node* n, string& out) {
    if (!n) { out += "#,"; return; }
    // escape commas/backslashes so values round-trip
    string esc;
    for (char c : n->val) { if (c == '\\' || c == ',') esc += '\\'; esc += c; }
    out += esc + ",";
    ser(n->left, out);
    ser(n->right, out);
}

string serialize(Node* root) { string s; ser(root, s); return s; }

Node* deser(const string& s, size_t& i) {
    if (i >= s.size()) return nullptr;
    string tok;
    bool isNull = (s[i] == '#');
    while (i < s.size() && s[i] != ',') {
        if (s[i] == '\\') { i++; if (i < s.size()) tok += s[i++]; }
        else tok += s[i++];
    }
    i++;  // skip comma
    if (isNull && tok == "#") return nullptr;
    Node* n = new Node(tok);
    n->left = deser(s, i);
    n->right = deser(s, i);
    return n;
}

Node* deserialize(const string& s) { size_t i = 0; return deser(s, i); }

int main() {
    Node* node = new Node("root", new Node("left", new Node("left.left")), new Node("right"));
    Node* round = deserialize(serialize(node));
    cout << (round->left->left->val == "left.left" ? "true" : "false") << "\n";
}

// Day 997: Serialize / deserialize a binary tree.
// Preorder traversal with "#" markers for null children, comma-joined.
// Both serialize and deserialize are O(n) time and space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    string val;
    Node *left = nullptr, *right = nullptr;
    Node(string v) : val(move(v)) {}
};

void ser(Node* n, string& out) {
    if (!n) { out += "#,"; return; }
    out += n->val + ",";
    ser(n->left, out);
    ser(n->right, out);
}

string serialize(Node* root) { string s; ser(root, s); return s; }

Node* deser(vector<string>& toks, size_t& i) {
    string v = toks[i++];
    if (v == "#") return nullptr;
    Node* n = new Node(v);
    n->left = deser(toks, i);
    n->right = deser(toks, i);
    return n;
}

Node* deserialize(const string& s) {
    vector<string> toks;
    string cur;
    for (char c : s) {
        if (c == ',') { toks.push_back(cur); cur.clear(); }
        else cur += c;
    }
    size_t i = 0;
    return deser(toks, i);
}

int main() {
    Node* node = new Node("root");
    node->left = new Node("left");
    node->left->left = new Node("left.left");
    node->right = new Node("right");
    string s = serialize(node);
    cout << s << "\n";
    cout << "assertion passed: " << deserialize(s)->left->left->val << "\n"; // left.left
    return 0;
}

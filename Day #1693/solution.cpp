// Ternary Search Tree: node has char + left/mid/right + isEnd. Compare char: <left, >right, ==mid & advance.
// Insert/search: O(L * log A) average where L=key length, A=alphabet size.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char c;
    bool isEnd = false;
    Node *left = nullptr, *mid = nullptr, *right = nullptr;
    Node(char ch) : c(ch) {}
};

Node* insert(Node* node, const string& s, int i) {
    char ch = s[i];
    if (!node) node = new Node(ch);
    if (ch < node->c) node->left = insert(node->left, s, i);
    else if (ch > node->c) node->right = insert(node->right, s, i);
    else if (i + 1 < (int)s.size()) node->mid = insert(node->mid, s, i + 1);
    else node->isEnd = true;
    return node;
}

bool search(Node* node, const string& s, int i) {
    if (!node) return false;
    char ch = s[i];
    if (ch < node->c) return search(node->left, s, i);
    if (ch > node->c) return search(node->right, s, i);
    if (i + 1 == (int)s.size()) return node->isEnd;
    return search(node->mid, s, i + 1);
}

int main() {
    Node* root = nullptr;
    vector<string> words = {"code", "cob", "be", "ax", "war", "we"};
    for (auto& w : words) root = insert(root, w, 0);

    vector<string> queries = {"code", "cob", "cod", "war", "wa"};
    for (auto& q : queries)
        cout << (search(root, q, 0) ? "true" : "false") << "\n";
    return 0;
}

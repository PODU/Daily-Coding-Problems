// Day 753: Ternary Search Tree with insert and search.
// Insert/Search: O(L + log n) average, O(L * n) worst; L = key length.
#include <bits/stdc++.h>
using namespace std;

struct TSTNode {
    char c;
    bool isEnd = false;
    TSTNode *left = nullptr, *mid = nullptr, *right = nullptr;
    TSTNode(char ch) : c(ch) {}
};

struct TST {
    TSTNode* root = nullptr;

    TSTNode* insert(TSTNode* node, const string& s, int i) {
        char c = s[i];
        if (!node) node = new TSTNode(c);
        if (c < node->c)      node->left  = insert(node->left,  s, i);
        else if (c > node->c) node->right = insert(node->right, s, i);
        else if (i + 1 < (int)s.size())
            node->mid = insert(node->mid, s, i + 1);
        else node->isEnd = true;
        return node;
    }
    void insert(const string& s) { if (!s.empty()) root = insert(root, s, 0); }

    bool search(const string& s) const {
        TSTNode* node = root;
        int i = 0;
        while (node) {
            char c = s[i];
            if (c < node->c)      node = node->left;
            else if (c > node->c) node = node->right;
            else {
                if (i + 1 == (int)s.size()) return node->isEnd;
                node = node->mid;
                i++;
            }
        }
        return false;
    }
};

int main() {
    TST tst;
    for (string w : {"code", "cob", "be", "ax", "war", "we"}) tst.insert(w);
    for (string q : {"code", "cob", "cod", "ax", "hello"})
        cout << q << ": " << (tst.search(q) ? "true" : "false") << "\n";
    return 0;
}

// Ternary Search Tree with insert/search. Each node: char + left/mid/right + end flag.
// Time: O(L * log A) per op, Space: O(total chars).
#include <iostream>
#include <string>
using namespace std;

struct Node {
    char c;
    bool end = false;
    Node *left = nullptr, *mid = nullptr, *right = nullptr;
    Node(char ch) : c(ch) {}
};

struct TST {
    Node* root = nullptr;

    Node* insert(Node* node, const string& w, int i) {
        char ch = w[i];
        if (!node) node = new Node(ch);
        if (ch < node->c) node->left = insert(node->left, w, i);
        else if (ch > node->c) node->right = insert(node->right, w, i);
        else if (i + 1 < (int)w.size()) node->mid = insert(node->mid, w, i + 1);
        else node->end = true;
        return node;
    }
    void insert(const string& w) { if (!w.empty()) root = insert(root, w, 0); }

    bool search(const string& w) const {
        Node* node = root;
        int i = 0;
        while (node) {
            char ch = w[i];
            if (ch < node->c) node = node->left;
            else if (ch > node->c) node = node->right;
            else {
                if (i + 1 == (int)w.size()) return node->end;
                i++;
                node = node->mid;
            }
        }
        return false;
    }
};

int main() {
    TST tst;
    for (string w : {"code", "cob", "be", "ax", "war", "we"}) tst.insert(w);
    for (string q : {"code", "cob", "ax", "c", "war", "cat"})
        cout << q << " -> " << (tst.search(q) ? "true" : "false") << "\n";
    return 0;
}

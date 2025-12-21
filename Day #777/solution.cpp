// Day 777: Ternary Search Tree with insert and search.
// Each node has left/mid/right children. O(L * log A) per op (L=word length).
#include <bits/stdc++.h>
using namespace std;

struct TST {
    struct Node { char c; bool end = false; Node *l=nullptr,*m=nullptr,*r=nullptr; Node(char ch):c(ch){} };
    Node* root = nullptr;

    Node* insert(Node* node, const string& w, int i) {
        char c = w[i];
        if (!node) node = new Node(c);
        if (c < node->c) node->l = insert(node->l, w, i);
        else if (c > node->c) node->r = insert(node->r, w, i);
        else {
            if (i + 1 < (int)w.size()) node->m = insert(node->m, w, i + 1);
            else node->end = true;
        }
        return node;
    }
    void insert(const string& w) { if (!w.empty()) root = insert(root, w, 0); }

    bool search(Node* node, const string& w, int i) {
        if (!node) return false;
        char c = w[i];
        if (c < node->c) return search(node->l, w, i);
        if (c > node->c) return search(node->r, w, i);
        if (i + 1 == (int)w.size()) return node->end;
        return search(node->m, w, i + 1);
    }
    bool search(const string& w) { return !w.empty() && search(root, w, 0); }
};

int main() {
    TST t;
    for (string w : {"code", "cob", "be", "ax", "war", "we"}) t.insert(w);
    cout << boolalpha;
    cout << t.search("cob") << " " << t.search("code") << " "
         << t.search("cod") << " " << t.search("we") << endl; // true true false true
    return 0;
}

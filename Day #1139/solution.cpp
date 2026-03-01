// LCA via parent pointers: equalize depths then walk up together. O(h) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *parent = nullptr, *left = nullptr, *right = nullptr;
    Node(int v) : val(v) {}
};

int depth(Node* n) {
    int d = 0;
    while (n->parent) { n = n->parent; d++; }
    return d;
}

Node* lca(Node* a, Node* b) {
    int da = depth(a), db = depth(b);
    while (da > db) { a = a->parent; da--; }
    while (db > da) { b = b->parent; db--; }
    while (a != b) { a = a->parent; b = b->parent; }
    return a;
}

Node* link(Node* p, Node* c, bool left) {
    if (left) p->left = c; else p->right = c;
    c->parent = p;
    return c;
}

int main() {
    Node* n1 = new Node(1);
    Node* n2 = new Node(2); Node* n3 = new Node(3);
    Node* n4 = new Node(4); Node* n5 = new Node(5);
    Node* n6 = new Node(6); Node* n7 = new Node(7);
    link(n1, n2, true); link(n1, n3, false);
    link(n2, n4, true); link(n2, n5, false);
    link(n3, n6, true); link(n3, n7, false);

    cout << lca(n4, n5)->val << "\n";
    cout << lca(n4, n6)->val << "\n";
    return 0;
}

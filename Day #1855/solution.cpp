// Day 1855: LCA in a binary tree with parent pointers.
// Two-pointer walk (like linked-list intersection): swap to the other node at root; meet at LCA. O(h) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left = nullptr, *right = nullptr, *parent = nullptr;
    Node(int v) : val(v) {}
};

Node* lca(Node* p, Node* q) {
    Node *a = p, *b = q;
    while (a != b) {
        a = a ? a->parent : q;
        b = b ? b->parent : p;
    }
    return a;
}

Node* attach(Node* parent, Node* child) { if (child) child->parent = parent; return child; }

int main() {
    Node* n1 = new Node(1);
    Node* n2 = new Node(2);
    Node* n3 = new Node(3);
    Node* n4 = new Node(4);
    Node* n5 = new Node(5);
    Node* n6 = new Node(6);
    Node* n7 = new Node(7);
    Node* n8 = new Node(8);
    n1->left = attach(n1, n2); n1->right = attach(n1, n3);
    n2->left = attach(n2, n4); n2->right = attach(n2, n5);
    n3->right = attach(n3, n6);
    n5->left = attach(n5, n7); n5->right = attach(n5, n8);

    cout << lca(n7, n8)->val << "\n"; // 5
    cout << lca(n7, n6)->val << "\n"; // 1
    cout << lca(n4, n8)->val << "\n"; // 2
    return 0;
}

// LCA with parent pointers: equalize depths, then walk both up together.
// Time O(h), Space O(1).
#include <iostream>
using namespace std;

struct Node {
    int val;
    Node* parent;
    Node* left;
    Node* right;
    Node(int v) : val(v), parent(nullptr), left(nullptr), right(nullptr) {}
};

int depth(Node* n) {
    int d = 0;
    while (n) { d++; n = n->parent; }
    return d;
}

Node* lca(Node* a, Node* b) {
    int da = depth(a), db = depth(b);
    while (da > db) { a = a->parent; da--; }
    while (db > da) { b = b->parent; db--; }
    while (a != b) { a = a->parent; b = b->parent; }
    return a;
}

Node* makeChild(Node* parent, Node* child) {
    if (child) child->parent = parent;
    return child;
}

int main() {
    Node* n3 = new Node(3);
    Node* n5 = new Node(5);
    Node* n1 = new Node(1);
    Node* n6 = new Node(6);
    Node* n2 = new Node(2);
    Node* n0 = new Node(0);
    Node* n8 = new Node(8);
    Node* n7 = new Node(7);
    Node* n4 = new Node(4);

    n3->left = makeChild(n3, n5);
    n3->right = makeChild(n3, n1);
    n5->left = makeChild(n5, n6);
    n5->right = makeChild(n5, n2);
    n1->left = makeChild(n1, n0);
    n1->right = makeChild(n1, n8);
    n2->left = makeChild(n2, n7);
    n2->right = makeChild(n2, n4);

    cout << lca(n6, n4)->val << "\n";
    cout << lca(n6, n8)->val << "\n";
    return 0;
}

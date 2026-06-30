// Tree node with parent pointer + locked_descendant_count; lock/unlock walk ancestors O(h).
// is_locked O(1). lock/unlock O(h). Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    Node* parent = nullptr;
    Node* left = nullptr;
    Node* right = nullptr;
    bool locked = false;
    int lockedDescendants = 0;

    bool isLocked() { return locked; }

    bool anyAncestorLocked() {
        for (Node* p = parent; p; p = p->parent)
            if (p->locked) return true;
        return false;
    }

    bool lock() {
        if (locked || lockedDescendants > 0 || anyAncestorLocked()) return false;
        locked = true;
        for (Node* p = parent; p; p = p->parent) p->lockedDescendants++;
        return true;
    }

    bool unlock() {
        if (!locked || lockedDescendants > 0 || anyAncestorLocked()) return false;
        locked = false;
        for (Node* p = parent; p; p = p->parent) p->lockedDescendants--;
        return true;
    }
};

Node* make(Node* parent) {
    Node* n = new Node();
    n->parent = parent;
    return n;
}

int main() {
    Node* root = make(nullptr);
    root->left = make(root);
    root->right = make(root);
    root->left->left = make(root->left);
    root->left->right = make(root->left);
    Node* L = root->left;
    Node* LL = root->left->left;

    cout << (LL->lock() ? "true" : "false") << "\n";
    cout << (L->lock() ? "true" : "false") << "\n";
    cout << (root->lock() ? "true" : "false") << "\n";
    cout << (LL->unlock() ? "true" : "false") << "\n";
    cout << (L->lock() ? "true" : "false") << "\n";
    cout << (root->lock() ? "true" : "false") << "\n";
    return 0;
}

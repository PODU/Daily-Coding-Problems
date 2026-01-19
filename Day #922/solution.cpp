// Locking binary tree: each node tracks lockedDescendants count; lock/unlock check
// ancestors + descendant count. All ops O(h) where h is tree height.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    Node* parent = nullptr;
    Node* left = nullptr;
    Node* right = nullptr;
    bool locked = false;
    int lockedDescendants = 0;

    bool is_locked() const { return locked; }

    bool anyAncestorLocked() {
        for (Node* p = parent; p; p = p->parent)
            if (p->locked) return true;
        return false;
    }

    bool lock() {
        if (locked) return false;
        if (lockedDescendants > 0) return false;
        if (anyAncestorLocked()) return false;
        locked = true;
        for (Node* p = parent; p; p = p->parent) p->lockedDescendants++;
        return true;
    }

    bool unlock() {
        if (!locked) return false;
        locked = false;
        for (Node* p = parent; p; p = p->parent) p->lockedDescendants--;
        return true;
    }
};

int main() {
    // root -> (a, b); a -> (c, d)
    Node root, a, b, c, d;
    root.left = &a; root.right = &b;
    a.parent = &root; b.parent = &root;
    a.left = &c; a.right = &d;
    c.parent = &a; d.parent = &a;

    cout << boolalpha;
    cout << "lock c (leaf)      -> " << c.lock()   << " (expect true)\n";
    cout << "lock a (ancestor)  -> " << a.lock()   << " (expect false)\n";
    cout << "lock root          -> " << root.lock()<< " (expect false)\n";
    cout << "unlock c           -> " << c.unlock() << " (expect true)\n";
    cout << "lock a             -> " << a.lock()   << " (expect true)\n";
    cout << "lock c (desc lock) -> " << c.lock()   << " (expect false)\n";
    cout << "unlock a           -> " << a.unlock() << " (expect true)\n";
    return 0;
}

// Day 1147: Locking in a binary tree.
// Node keeps parent + count of locked descendants; lock/unlock walk ancestors. O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    Node *left = nullptr, *right = nullptr, *parent = nullptr;
    bool locked = false;
    int lockedDesc = 0;

    bool is_locked() const { return locked; }

    bool canLock() const {
        if (locked || lockedDesc > 0) return false;
        for (Node* a = parent; a; a = a->parent) if (a->locked) return false;
        return true;
    }

    bool lock() {
        if (!canLock()) return false;
        locked = true;
        for (Node* a = parent; a; a = a->parent) a->lockedDesc++;
        return true;
    }

    bool unlock() {
        if (!locked) return false;
        locked = false;
        for (Node* a = parent; a; a = a->parent) a->lockedDesc--;
        return true;
    }
};

int main() {
    Node root, l, r, ll;
    root.left = &l; root.right = &r; l.parent = &root; r.parent = &root;
    l.left = &ll; ll.parent = &l;
    cout << boolalpha;
    cout << l.lock() << "\n";      // true
    cout << ll.lock() << "\n";     // false (ancestor l locked)
    cout << root.lock() << "\n";   // false (descendant l locked)
    cout << l.unlock() << "\n";    // true
    cout << ll.lock() << "\n";     // true
    return 0;
}

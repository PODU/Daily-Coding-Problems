// Tree locking with parent pointers + per-node lockedDescendantCount.
// lock/unlock are O(h): walk ancestors once to check, once to update counts.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    string name;
    Node* parent;
    Node* left;
    Node* right;
    bool locked;
    int lockedDescendantCount;
    Node(string n) : name(n), parent(nullptr), left(nullptr), right(nullptr),
                     locked(false), lockedDescendantCount(0) {}

    bool isLocked() { return locked; }

    // True if some ancestor is locked.
    bool anyAncestorLocked() {
        for (Node* p = parent; p; p = p->parent)
            if (p->locked) return true;
        return false;
    }

    bool lock() {
        if (locked) return false;
        if (lockedDescendantCount > 0) return false;   // a descendant is locked
        if (anyAncestorLocked()) return false;         // an ancestor is locked
        locked = true;
        for (Node* p = parent; p; p = p->parent) p->lockedDescendantCount++;
        return true;
    }

    bool unlock() {
        if (!locked) return false;
        locked = false;
        for (Node* p = parent; p; p = p->parent) p->lockedDescendantCount--;
        return true;
    }
};

Node* child(Node* p, Node* c, bool left) {
    if (left) p->left = c; else p->right = c;
    c->parent = p;
    return c;
}

int main() {
    Node* n1 = new Node("node1");
    Node* n2 = new Node("node2");
    Node* n3 = new Node("node3");
    Node* n4 = new Node("node4");
    Node* n5 = new Node("node5");
    child(n1, n2, true); child(n1, n3, false);
    child(n2, n4, true); child(n2, n5, false);

    auto cap = [](bool b) { return b ? "True" : "False"; };
    cout << "lock(node4): " << cap(n4->lock()) << "\n";        // True
    cout << "lock(node2): " << cap(n2->lock()) << "\n";        // False (descendant locked)
    cout << "unlock(node4): " << cap(n4->unlock()) << "\n";    // True
    cout << "lock(node2): " << cap(n2->lock()) << "\n";        // True
    cout << "lock(node1): " << cap(n1->lock()) << "\n";        // False (descendant locked)
    cout << "lock(node5): " << cap(n5->lock()) << "\n";        // False (ancestor locked)
    cout << "unlock(node2): " << cap(n2->unlock()) << "\n";    // True
    cout << "lock(node1): " << cap(n1->lock()) << "\n";        // True
    return 0;
}

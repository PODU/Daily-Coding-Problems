// Binary tree locking: each node has a parent pointer and lockedDescendantCount.
// lock/unlock check ancestors (O(h)) + descendant count, then update ancestors (O(h)).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    string name;
    Node* parent;
    Node* left;
    Node* right;
    bool locked;
    int lockedDescendantCount;
    Node(string n, Node* p = nullptr)
        : name(n), parent(p), left(nullptr), right(nullptr), locked(false), lockedDescendantCount(0) {}

    bool is_locked() { return locked; }

    bool canLock() {
        if (lockedDescendantCount > 0) return false;
        for (Node* cur = parent; cur; cur = cur->parent)
            if (cur->locked) return false;
        return true;
    }

    bool lock() {
        if (locked) return false;
        if (!canLock()) return false;
        locked = true;
        for (Node* cur = parent; cur; cur = cur->parent)
            cur->lockedDescendantCount++;
        return true;
    }

    bool unlock() {
        if (!locked) return false;
        locked = false;
        for (Node* cur = parent; cur; cur = cur->parent)
            cur->lockedDescendantCount--;
        return true;
    }
};

int main() {
    Node* root = new Node("root");
    Node* node1 = new Node("node1", root);
    Node* node2 = new Node("node2", root);
    root->left = node1;
    root->right = node2;
    Node* node3 = new Node("node3", node1);
    Node* node4 = new Node("node4", node1);
    node1->left = node3;
    node1->right = node4;

    cout << "node2.lock() = " << (node2->lock() ? "true" : "false") << endl;
    cout << "root.lock() = " << (root->lock() ? "true" : "false") << endl;
    cout << "node2.unlock() = " << (node2->unlock() ? "true" : "false") << endl;
    cout << "root.lock() = " << (root->lock() ? "true" : "false") << endl;
    return 0;
}

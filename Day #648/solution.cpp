// Deep clone linked list with random pointers using O(1) interleaving (3 passes).
// Time O(n), Space O(1) extra. C++17.
#include <iostream>
#include <unordered_set>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node* random;
    Node(int v): val(v), next(nullptr), random(nullptr) {}
};

Node* cloneList(Node* head) {
    if (!head) return nullptr;
    // Pass 1: insert cloned node after each original
    for (Node* cur = head; cur; cur = cur->next->next) {
        Node* copy = new Node(cur->val);
        copy->next = cur->next;
        cur->next = copy;
    }
    // Pass 2: set clone.random
    for (Node* cur = head; cur; cur = cur->next->next) {
        cur->next->random = cur->random ? cur->random->next : nullptr;
    }
    // Pass 3: split lists
    Node* newHead = head->next;
    for (Node* cur = head; cur; cur = cur->next) {
        Node* copy = cur->next;
        cur->next = copy->next;
        copy->next = copy->next ? copy->next->next : nullptr;
    }
    return newHead;
}

int main() {
    Node* n1 = new Node(1);
    Node* n2 = new Node(2);
    Node* n3 = new Node(3);
    Node* n4 = new Node(4);
    n1->next = n2; n2->next = n3; n3->next = n4;
    n1->random = n3;
    n2->random = n1;
    n3->random = n3;
    n4->random = n2;

    Node* cloned = cloneList(n1);

    // collect original node pointers
    unordered_set<Node*> origSet;
    for (Node* cur = n1; cur; cur = cur->next) origSet.insert(cur);

    bool deep = true;
    for (Node* cur = cloned; cur; cur = cur->next) {
        cout << "node " << cur->val << " random "
             << (cur->random ? cur->random->val : 0) << "\n";
        if (origSet.count(cur)) deep = false;
        if (cur->random && origSet.count(cur->random)) deep = false;
    }
    cout << "deep copy verified: " << (deep ? "true" : "false") << "\n";
    return 0;
}

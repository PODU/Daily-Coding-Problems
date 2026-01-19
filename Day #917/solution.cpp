// Partition linked list: stable split into <k and >=k lists, then concatenate.
// Time O(n), Space O(1).
#include <iostream>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* partition(Node* head, int k) {
    Node lessDummy(0), geDummy(0);
    Node* lt = &lessDummy; Node* ge = &geDummy;
    for (Node* cur = head; cur; cur = cur->next) {
        if (cur->val < k) { lt->next = cur; lt = cur; }
        else { ge->next = cur; ge = cur; }
    }
    ge->next = nullptr;
    lt->next = geDummy.next;
    return lessDummy.next;
}

int main() {
    int vals[] = {5,1,8,0,3};
    Node* head = nullptr; Node* tail = nullptr;
    for (int v : vals) {
        Node* n = new Node(v);
        if (!head) head = tail = n; else { tail->next = n; tail = n; }
    }
    head = partition(head, 3);
    for (Node* cur = head; cur; cur = cur->next) {
        cout << cur->val;
        if (cur->next) cout << " -> ";
    }
    cout << endl;
    return 0;
}

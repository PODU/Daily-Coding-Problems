// Remove kth-from-last node in one pass with two pointers. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

Node* removeKthFromLast(Node* head, int k) {
    Node dummy(0);
    dummy.next = head;
    Node* lead = &dummy;
    Node* trail = &dummy;
    for (int i = 0; i < k; ++i) lead = lead->next;
    while (lead->next) {
        lead = lead->next;
        trail = trail->next;
    }
    Node* toDel = trail->next;
    trail->next = toDel->next;
    delete toDel;
    return dummy.next;
}

int main() {
    Node* head = new Node(1);
    head->next = new Node(2);
    head->next->next = new Node(3);
    head->next->next->next = new Node(4);
    head->next->next->next->next = new Node(5);
    head = removeKthFromLast(head, 2);
    for (Node* c = head; c; c = c->next) {
        cout << c->val;
        if (c->next) cout << " -> ";
    }
    cout << "\n";
    return 0;
}

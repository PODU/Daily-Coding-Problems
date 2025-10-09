// Remove k-th node from end in one pass via two pointers + dummy head. O(n) time, O(1) space.
#include <iostream>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* removeKthFromEnd(Node* head, int k) {
    Node dummy(0); dummy.next = head;
    Node* fast = &dummy; Node* slow = &dummy;
    for (int i = 0; i < k; ++i) fast = fast->next; // advance fast k ahead
    while (fast->next) { fast = fast->next; slow = slow->next; }
    Node* del = slow->next;
    slow->next = del->next;
    delete del;
    return dummy.next;
}

void printList(Node* head) {
    for (Node* c = head; c; c = c->next) {
        cout << c->val;
        if (c->next) cout << " -> ";
    }
    cout << "\n";
}

int main() {
    Node* head = new Node(1);
    head->next = new Node(2);
    head->next->next = new Node(3);
    head->next->next->next = new Node(4);
    head->next->next->next->next = new Node(5);
    head = removeKthFromEnd(head, 2);
    printList(head);
    return 0;
}

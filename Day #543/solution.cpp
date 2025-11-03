// Remove kth-from-last node in one pass via two pointers k apart. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* removeKthLast(Node* head, int k) {
    Node dummy(0); dummy.next = head;
    Node* fast = &dummy; Node* slow = &dummy;
    for (int i = 0; i < k; i++) fast = fast->next; // advance fast by k
    while (fast->next) { fast = fast->next; slow = slow->next; }
    Node* del = slow->next;
    slow->next = del->next;
    delete del;
    return dummy.next;
}

int main() {
    Node* head = new Node(1);
    head->next = new Node(2);
    head->next->next = new Node(3);
    head->next->next->next = new Node(4);
    head->next->next->next->next = new Node(5);
    head = removeKthLast(head, 2);
    bool first = true;
    for (Node* p = head; p; p = p->next) { if (!first) cout << ' '; cout << p->val; first = false; }
    cout << "\n";
    return 0;
}

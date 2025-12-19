// Day 765: Remove the kth-from-last node in one pass with two pointers.
// fast leads slow by k; when fast hits the end, slow precedes the target.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* removeKthLast(Node* head, int k) {
    Node dummy(0); dummy.next = head;
    Node* fast = &dummy; Node* slow = &dummy;
    for (int i = 0; i < k; ++i) fast = fast->next;   // lead by k
    while (fast->next) { fast = fast->next; slow = slow->next; }
    Node* target = slow->next;
    slow->next = target->next;                       // unlink
    delete target;
    return dummy.next;
}

void printList(Node* head) {
    for (Node* p = head; p; p = p->next)
        cout << p->val << (p->next ? " -> " : "\n");
}

int main() {
    Node* head = new Node(1);
    Node* cur = head;
    for (int v = 2; v <= 5; ++v) { cur->next = new Node(v); cur = cur->next; }

    cout << "before: "; printList(head);
    head = removeKthLast(head, 2);     // remove value 4
    cout << "after:  "; printList(head);   // 1 -> 2 -> 3 -> 5
    return 0;
}

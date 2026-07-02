// Day 1752: Remove kth-from-last node of a singly linked list in ONE pass, O(1) space.
// Two pointers spaced k apart; when fast reaches end, slow is just before the target. O(n) time.
// Assumption (no README example): list 1->2->3->4->5, k=2 removes value 4 -> "1 2 3 5".
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v): val(v), next(nullptr) {}
};

Node* removeKthLast(Node* head, int k) {
    Node dummy(0);
    dummy.next = head;
    Node* fast = &dummy;
    Node* slow = &dummy;
    for (int i = 0; i < k; ++i) fast = fast->next; // advance fast k steps
    while (fast->next) { fast = fast->next; slow = slow->next; }
    Node* target = slow->next;
    slow->next = target->next;
    delete target;
    return dummy.next;
}

int main() {
    Node* head = new Node(1);
    head->next = new Node(2);
    head->next->next = new Node(3);
    head->next->next->next = new Node(4);
    head->next->next->next->next = new Node(5);

    head = removeKthLast(head, 2);
    for (Node* p = head; p; p = p->next)
        cout << p->val << (p->next ? " " : "\n"); // 1 2 3 5
    return 0;
}

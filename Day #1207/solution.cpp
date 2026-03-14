// Day 1207: Remove kth-from-last node in one pass, constant space.
// Two pointers k apart; when lead hits end, trail is just before target. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* removeKthLast(Node* head, int k) {
    Node dummy(0); dummy.next = head;
    Node* lead = &dummy; Node* trail = &dummy;
    for (int i = 0; i < k; i++) lead = lead->next;
    while (lead->next) { lead = lead->next; trail = trail->next; }
    trail->next = trail->next->next; // remove target
    return dummy.next;
}

int main() {
    Node* head = new Node(1);
    head->next = new Node(2); head->next->next = new Node(3);
    head->next->next->next = new Node(4); head->next->next->next->next = new Node(5);
    head = removeKthLast(head, 2); // remove 4 -> 1 2 3 5
    string out;
    for (Node* p = head; p; p = p->next) { if (!out.empty()) out += " -> "; out += to_string(p->val); }
    cout << out << "\n";
    return 0;
}

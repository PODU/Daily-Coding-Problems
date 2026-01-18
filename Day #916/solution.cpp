// Reverse a singly linked list in-place by re-pointing each node's next pointer.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* reverse(Node* head) {
    Node* prev = nullptr;
    while (head) { Node* nx = head->next; head->next = prev; prev = head; head = nx; }
    return prev;
}

void print(Node* h) {
    bool first = true;
    while (h) { if (!first) cout << " -> "; cout << h->val; first = false; h = h->next; }
    cout << "\n";
}

int main() {
    Node* head = new Node(1);
    head->next = new Node(2);
    head->next->next = new Node(3);
    head->next->next->next = new Node(4);
    head->next->next->next->next = new Node(5);
    head = reverse(head);
    print(head); // 5 -> 4 -> 3 -> 2 -> 1
    return 0;
}

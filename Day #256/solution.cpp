// Day 256: Rearrange linked list values into zigzag low->high->low form.
// One pass over node values: at even i ensure a[i]<=a[i+1], at odd i ensure a[i]>=a[i+1], swap on violation.
// Time: O(n), Space: O(1).
#include <iostream>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

void wiggle(Node* head) {
    bool less = true; // even index: want current <= next
    for (Node* cur = head; cur && cur->next; cur = cur->next) {
        if (less) {
            if (cur->val > cur->next->val) swap(cur->val, cur->next->val);
        } else {
            if (cur->val < cur->next->val) swap(cur->val, cur->next->val);
        }
        less = !less;
    }
}

void printList(Node* head) {
    for (Node* cur = head; cur; cur = cur->next) {
        cout << cur->val;
        if (cur->next) cout << " -> ";
    }
    cout << "\n";
}

int main() {
    int vals[] = {1, 2, 3, 4, 5};
    Node* head = nullptr;
    Node* tail = nullptr;
    for (int v : vals) {
        Node* n = new Node(v);
        if (!head) head = tail = n;
        else { tail->next = n; tail = n; }
    }
    wiggle(head);
    printList(head); // 1 -> 3 -> 2 -> 5 -> 4
    return 0;
}

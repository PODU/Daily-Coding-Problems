// Day 1180: Swap every two adjacent nodes in a singly linked list.
// Iterative pointer rewiring with a dummy head. Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct ListNode { int val; ListNode* next; ListNode(int v): val(v), next(nullptr) {} };

ListNode* swapPairs(ListNode* head) {
    ListNode dummy(0); dummy.next = head;
    ListNode* prev = &dummy;
    while (prev->next && prev->next->next) {
        ListNode* a = prev->next;
        ListNode* b = a->next;
        a->next = b->next;
        b->next = a;
        prev->next = b;
        prev = a;
    }
    return dummy.next;
}

ListNode* build(vector<int> v) {
    ListNode dummy(0); ListNode* t = &dummy;
    for (int x : v) { t->next = new ListNode(x); t = t->next; }
    return dummy.next;
}

void print(ListNode* h) {
    for (; h; h = h->next) cout << h->val << (h->next ? " -> " : "\n");
}

int main() {
    ListNode* head = build({1, 2, 3, 4});
    print(swapPairs(head)); // 2 -> 1 -> 4 -> 3
    return 0;
}

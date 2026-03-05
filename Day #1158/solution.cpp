// Bottom-up iterative merge sort on a singly linked list. O(n log n) time, O(1) auxiliary space.
#include <bits/stdc++.h>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int v) : val(v), next(nullptr) {}
};

// Split list, return node after taking `n` nodes; truncates the first part.
ListNode* split(ListNode* head, int n) {
    for (int i = 1; head && i < n; ++i) head = head->next;
    if (!head) return nullptr;
    ListNode* second = head->next;
    head->next = nullptr;
    return second;
}

// Merge two sorted lists onto tail, return new tail.
ListNode* merge(ListNode* a, ListNode* b, ListNode* tail) {
    ListNode* cur = tail;
    while (a && b) {
        if (a->val <= b->val) { cur->next = a; a = a->next; }
        else { cur->next = b; b = b->next; }
        cur = cur->next;
    }
    cur->next = a ? a : b;
    while (cur->next) cur = cur->next;
    return cur;
}

ListNode* sortList(ListNode* head) {
    if (!head || !head->next) return head;
    int n = 0;
    for (ListNode* p = head; p; p = p->next) ++n;

    ListNode dummy(0);
    dummy.next = head;
    for (int size = 1; size < n; size <<= 1) {
        ListNode* cur = dummy.next;
        ListNode* tail = &dummy;
        while (cur) {
            ListNode* left = cur;
            ListNode* right = split(left, size);
            cur = split(right, size);
            tail = merge(left, right, tail);
        }
    }
    return dummy.next;
}

int main() {
    int vals[] = {4, 1, -3, 99};
    ListNode* head = nullptr;
    ListNode* tail = nullptr;
    for (int v : vals) {
        ListNode* node = new ListNode(v);
        if (!head) head = tail = node;
        else { tail->next = node; tail = node; }
    }

    head = sortList(head);

    bool first = true;
    for (ListNode* p = head; p; p = p->next) {
        if (!first) cout << " -> ";
        cout << p->val;
        first = false;
    }
    cout << "\n";
    return 0;
}

// Add two numbers stored as reversed-digit linked lists via elementary addition
// with carry, walking both lists. Time O(max(m,n)), Space O(max(m,n)).
#include <bits/stdc++.h>
using namespace std;

struct ListNode { int val; ListNode* next; ListNode(int v): val(v), next(nullptr) {} };

ListNode* build(vector<int> d) {
    ListNode dummy(0), *cur = &dummy;
    for (int x : d) { cur->next = new ListNode(x); cur = cur->next; }
    return dummy.next;
}

ListNode* addLists(ListNode* a, ListNode* b) {
    ListNode dummy(0), *cur = &dummy;
    int carry = 0;
    while (a || b || carry) {
        int s = carry + (a ? a->val : 0) + (b ? b->val : 0);
        carry = s / 10;
        cur->next = new ListNode(s % 10);
        cur = cur->next;
        if (a) a = a->next;
        if (b) b = b->next;
    }
    return dummy.next;
}

void print(ListNode* n) {
    bool first = true;
    while (n) { if (!first) cout << " -> "; cout << n->val; first = false; n = n->next; }
    cout << "\n";
}

int main() {
    ListNode* a = build({9, 9}); // 99
    ListNode* b = build({5, 2}); // 25
    print(addLists(a, b));       // 124 -> 4 -> 2 -> 1
    return 0;
}

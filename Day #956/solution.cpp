// Day 956: merge k sorted singly linked lists using a min-heap.
// Time O(N log k), Space O(k) where N = total nodes.
#include <bits/stdc++.h>
using namespace std;

struct ListNode { int val; ListNode* next; ListNode(int v): val(v), next(nullptr) {} };

ListNode* mergeK(vector<ListNode*>& lists) {
    auto cmp = [](ListNode* a, ListNode* b) { return a->val > b->val; };
    priority_queue<ListNode*, vector<ListNode*>, decltype(cmp)> pq(cmp);
    for (ListNode* l : lists) if (l) pq.push(l);
    ListNode dummy(0); ListNode* tail = &dummy;
    while (!pq.empty()) {
        ListNode* n = pq.top(); pq.pop();
        tail->next = n; tail = n;
        if (n->next) pq.push(n->next);
    }
    return dummy.next;
}

ListNode* build(vector<int> v) {
    ListNode dummy(0); ListNode* t = &dummy;
    for (int x : v) { t->next = new ListNode(x); t = t->next; }
    return dummy.next;
}

int main() {
    vector<ListNode*> lists = { build({1,4,5}), build({1,3,4}), build({2,6}) };
    ListNode* m = mergeK(lists);
    for (ListNode* p = m; p; p = p->next) cout << p->val << (p->next ? " " : "\n");
    // 1 1 2 3 4 4 5 6
    return 0;
}

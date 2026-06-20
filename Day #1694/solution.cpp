// Zigzag list: single pass, even index wants lst[i]<=lst[i+1], odd wants lst[i]>=lst[i+1]; swap if violated.
// O(n) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int v) : val(v), next(nullptr) {}
};

void zigzag(ListNode* head) {
    ListNode* cur = head;
    int i = 0;
    while (cur && cur->next) {
        if (i % 2 == 0) {
            if (cur->val > cur->next->val) swap(cur->val, cur->next->val);
        } else {
            if (cur->val < cur->next->val) swap(cur->val, cur->next->val);
        }
        cur = cur->next;
        i++;
    }
}

int main() {
    ListNode* head = new ListNode(1);
    ListNode* cur = head;
    for (int v : {2, 3, 4, 5}) { cur->next = new ListNode(v); cur = cur->next; }

    zigzag(head);

    for (cur = head; cur; cur = cur->next) {
        cout << cur->val;
        if (cur->next) cout << " -> ";
    }
    cout << "\n";
    return 0;
}

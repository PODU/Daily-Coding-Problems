// Day 715: Linked-list palindrome. Find middle (slow/fast), reverse second half,
// compare. Works for singly linked in O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

bool isPalindrome(Node* head) {
    if (!head || !head->next) return true;
    Node *slow = head, *fast = head;
    while (fast->next && fast->next->next) { slow = slow->next; fast = fast->next->next; }
    // reverse second half
    Node* prev = nullptr;
    Node* cur = slow->next;
    while (cur) { Node* nx = cur->next; cur->next = prev; prev = cur; cur = nx; }
    Node *p1 = head, *p2 = prev;
    bool ok = true;
    while (p2) { if (p1->val != p2->val) { ok = false; break; } p1 = p1->next; p2 = p2->next; }
    return ok;
}

Node* build(vector<int> v) {
    Node dummy(0); Node* t = &dummy;
    for (int x : v) { t->next = new Node(x); t = t->next; }
    return dummy.next;
}

int main() {
    cout << (isPalindrome(build({1, 4, 3, 4, 1})) ? "True" : "False") << endl;
    cout << (isPalindrome(build({1, 4})) ? "True" : "False") << endl;
    return 0;
}

// Day 1677: Linked-list palindrome. Singly: find middle, reverse 2nd half, compare.
// Doubly: two pointers from both ends. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node* prev; Node(int v): val(v), next(nullptr), prev(nullptr) {} };

Node* build(const vector<int>& v, bool doubly) {
    Node* head = nullptr; Node* tail = nullptr;
    for (int x : v) {
        Node* n = new Node(x);
        if (!head) head = tail = n;
        else { tail->next = n; if (doubly) n->prev = tail; tail = n; }
    }
    return head;
}

// Works for singly linked (also valid for doubly).
bool isPalindrome(Node* head) {
    Node* slow = head; Node* fast = head;
    while (fast && fast->next) { slow = slow->next; fast = fast->next->next; }
    // reverse second half
    Node* prev = nullptr;
    while (slow) { Node* nx = slow->next; slow->next = prev; prev = slow; slow = nx; }
    Node* a = head; Node* b = prev;
    while (b) { if (a->val != b->val) return false; a = a->next; b = b->next; }
    return true;
}

int main() {
    cout << (isPalindrome(build({1,4,3,4,1}, false)) ? "True" : "False") << "\n"; // True
    cout << (isPalindrome(build({1,4}, false)) ? "True" : "False") << "\n";       // False
    return 0;
}

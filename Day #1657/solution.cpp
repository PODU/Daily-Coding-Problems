// Rotate singly linked list right by k. Make ring, break at n-(k%n). O(n) time, O(1) space.
#include <iostream>
using namespace std;

struct Node { int v; Node* next; Node(int x):v(x),next(nullptr){} };

Node* rotateRight(Node* head, int k) {
    if (!head || !head->next) return head;
    int n = 1; Node* tail = head;
    while (tail->next) { tail = tail->next; ++n; }
    tail->next = head;                 // ring
    int steps = n - (k % n);
    Node* newTail = head;
    for (int i = 1; i < steps; ++i) newTail = newTail->next;
    Node* newHead = newTail->next;
    newTail->next = nullptr;
    return newHead;
}

Node* build(initializer_list<int> xs) {
    Node *h=nullptr,*t=nullptr;
    for (int x: xs){ Node* nd=new Node(x); if(!h)h=t=nd; else {t->next=nd;t=nd;} }
    return h;
}
void print(Node* h){ for(Node* p=h;p;p=p->next){ cout<<p->v; if(p->next)cout<<" -> "; } cout<<"\n"; }

int main(){
    print(rotateRight(build({7,7,3,5}), 2));
    print(rotateRight(build({1,2,3,4,5}), 3));
    return 0;
}

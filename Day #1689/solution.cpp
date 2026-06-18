// Sort a singly linked list via bottom-up (iterative) merge sort.
// O(n log n) time, O(1) extra space (no recursion).
#include <iostream>
using namespace std;

struct Node { int val; Node* next; Node(int v):val(v),next(nullptr){} };

int length(Node* h){ int n=0; while(h){ n++; h=h->next; } return n; }

// Cut list to first n nodes; return head of the remainder.
Node* split(Node* head, int n){
    for(int i=1; head && i<n; i++) head=head->next;
    if(!head) return nullptr;
    Node* second=head->next;
    head->next=nullptr;
    return second;
}

Node* merge(Node* a, Node* b){
    Node dummy(0); Node* tail=&dummy;
    while(a && b){
        if(a->val<=b->val){ tail->next=a; a=a->next; }
        else { tail->next=b; b=b->next; }
        tail=tail->next;
    }
    tail->next = a ? a : b;
    return dummy.next;
}

Node* sortList(Node* head){
    if(!head) return nullptr;
    int n=length(head);
    Node dummy(0); dummy.next=head;
    for(int size=1; size<n; size<<=1){
        Node* prev=&dummy; Node* cur=dummy.next;
        while(cur){
            Node* left=cur;
            Node* right=split(left,size);
            cur=split(right,size);
            prev->next=merge(left,right);
            while(prev->next) prev=prev->next;
        }
    }
    return dummy.next;
}

int main(){
    int vals[]={4,1,-3,99};
    Node* head=nullptr; Node* tail=nullptr;
    for(int v:vals){ Node* nd=new Node(v); if(!head){head=tail=nd;} else {tail->next=nd;tail=nd;} }
    head=sortList(head);
    bool first=true;
    for(Node* p=head;p;p=p->next){ if(!first) cout<<" -> "; cout<<p->val; first=false; }
    cout<<"\n";
    return 0;
}

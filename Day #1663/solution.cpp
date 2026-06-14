// Fisher-Yates shuffle of linked-list node values via array: O(n) time, O(n) space.
// Space-over-time tradeoff: an O(1)-extra approach repeatedly picks a random node by traversal in O(n^2) time.
#include <bits/stdc++.h>
using namespace std;
struct Node { int val; Node* next; Node(int v):val(v),next(nullptr){} };
int main(){
    // build 1->2->3->4->5
    Node* head=nullptr; Node* tail=nullptr;
    for(int v=1; v<=5; ++v){ Node* n=new Node(v); if(!head) head=tail=n; else { tail->next=n; tail=n; } }
    // collect pointers
    vector<Node*> a; for(Node* p=head;p;p=p->next) a.push_back(p);
    int n=a.size();
    mt19937 rng(12345);
    // Fisher-Yates
    for(int i=n-1;i>0;--i){ int j=rng()%(i+1); swap(a[i],a[j]); }
    // relink shuffled
    for(int i=0;i<n;++i) a[i]->next=(i+1<n?a[i+1]:nullptr);
    head=a[0];
    // verify valid permutation
    vector<int> orig, shuf;
    for(int v=1;v<=5;++v) orig.push_back(v);
    for(Node* p=head;p;p=p->next) shuf.push_back(p->val);
    sort(shuf.begin(),shuf.end());
    bool valid=(orig==shuf);
    for(int i=0;i<(int)orig.size();++i){ if(i) cout<<' '; cout<<orig[i]; }
    cout<<" -> "<<(valid?"valid shuffle (same elements)":"INVALID")<<"\n";
    return 0;
}

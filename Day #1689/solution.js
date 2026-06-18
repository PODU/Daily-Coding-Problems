// Sort a singly linked list via bottom-up (iterative) merge sort.
// O(n log n) time, O(1) extra space (no recursion).
'use strict';

class Node { constructor(val){ this.val = val; this.next = null; } }

function length(h){ let n=0; while(h){ n++; h=h.next; } return n; }

// Cut list to first n nodes; return head of the remainder.
function split(head, n){
    for(let i=1; head && i<n; i++) head=head.next;
    if(!head) return null;
    const second=head.next; head.next=null; return second;
}

function merge(a, b){
    const dummy=new Node(0); let tail=dummy;
    while(a && b){
        if(a.val<=b.val){ tail.next=a; a=a.next; }
        else { tail.next=b; b=b.next; }
        tail=tail.next;
    }
    tail.next = a || b;
    return dummy.next;
}

function sortList(head){
    if(!head) return null;
    const n=length(head);
    const dummy=new Node(0); dummy.next=head;
    for(let size=1; size<n; size<<=1){
        let prev=dummy, cur=dummy.next;
        while(cur){
            const left=cur;
            const right=split(left,size);
            cur=split(right,size);
            prev.next=merge(left,right);
            while(prev.next) prev=prev.next;
        }
    }
    return dummy.next;
}

let head=null, tail=null;
for(const v of [4,1,-3,99]){ const nd=new Node(v); if(!head){head=tail=nd;} else {tail.next=nd;tail=nd;} }
head=sortList(head);
const parts=[];
for(let p=head;p;p=p.next) parts.push(p.val);
console.log(parts.join(" -> "));

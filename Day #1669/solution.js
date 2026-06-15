// Two-pointer merge from both ends of sorted array; larger abs square goes to back. O(n) time, O(n) space.
function sortedSquares(a){
    const n=a.length,r=new Array(n);let l=0,h=n-1;
    for(let p=n-1;p>=0;--p){const lo=a[l]*a[l],hi=a[h]*a[h];
        if(lo>hi){r[p]=lo;++l;}else{r[p]=hi;--h;}}
    return r;
}
console.log("["+sortedSquares([-9,-2,0,2,3]).join(", ")+"]");

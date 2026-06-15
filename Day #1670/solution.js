// Prefix-sum hashmap: store sum->index; if sum-K seen, slice between. O(n) time, O(n) space.
function subarraySum(a, K){
    const seen=new Map([[0,-1]]);let s=0;
    for(let i=0;i<a.length;++i){s+=a[i];
        if(seen.has(s-K))return a.slice(seen.get(s-K)+1,i+1);
        if(!seen.has(s))seen.set(s,i);}
    return [];
}
console.log("["+subarraySum([1,2,3,4,5],9).join(", ")+"]");

// Closest pair of points via divide & conquer. O(n log n) time, O(n) space.
// Sort by x, recurse on halves, merge by checking a y-sorted strip near the split.
'use strict';

function dist(a, b){ return Math.hypot(a[0]-b[0], a[1]-b[1]); }

function closest(px, py){
    const n = px.length;
    if(n <= 3){
        let best = [Infinity, null, null];
        for(let i=0;i<n;i++) for(let j=i+1;j<n;j++){
            const d = dist(px[i], px[j]);
            if(d < best[0]) best = [d, px[i], px[j]];
        }
        return best;
    }
    const mid = n>>1;
    const midx = px[mid][0];
    const lx = px.slice(0, mid), rx = px.slice(mid);
    const leftCount = new Map();
    for(const p of lx){ const k=p[0]+','+p[1]; leftCount.set(k,(leftCount.get(k)||0)+1); }
    const ly=[], ry=[];
    for(const p of py){
        const k=p[0]+','+p[1]; const c=leftCount.get(k)||0;
        if(c>0){ ly.push(p); leftCount.set(k,c-1); } else ry.push(p);
    }
    const bl = closest(lx, ly), br = closest(rx, ry);
    let best = bl[0] < br[0] ? bl : br;
    let d = best[0];
    const strip = py.filter(p => Math.abs(p[0]-midx) < d);
    for(let i=0;i<strip.length;i++)
        for(let j=i+1;j<strip.length && (strip[j][1]-strip[i][1])<d;j++){
            const dd = dist(strip[i], strip[j]);
            if(dd<d){ d=dd; best=[dd, strip[i], strip[j]]; }
        }
    return best;
}

function main(){
    const pts = [[1,1],[-1,-1],[3,4],[6,1],[-1,-6],[-4,-3]];
    const px = [...pts].sort((a,b)=> a[0]-b[0] || a[1]-b[1]);
    const py = [...pts].sort((a,b)=> a[1]-b[1] || a[0]-b[0]);
    let [, a, b] = closest(px, py);
    [a, b] = [a, b].sort((p,q)=> p[0]-q[0] || p[1]-q[1]);
    console.log(`[(${a[0]}, ${a[1]}), (${b[0]}, ${b[1]})]`);
}

main();

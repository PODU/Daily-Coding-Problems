// Boggle solver: build a trie from the dictionary, DFS 8-directionally from each
// cell following trie edges with a visited mask. O(cells * 8^L) worst, pruned by trie.
'use strict';

function main(){
    const rows = ["oaan","etae","ihkr","iflv"];
    const dict = ["oath","pea","eat","rain"];

    const root = { ch: {}, end: false };
    for(const w of dict){
        let nd=root;
        for(const c of w){ if(!nd.ch[c]) nd.ch[c]={ch:{},end:false}; nd=nd.ch[c]; }
        nd.end=true;
    }

    const grid = rows.map(r => r.split(''));
    const R=grid.length, C=grid[0].length;
    const vis = Array.from({length:R}, () => new Array(C).fill(false));
    const found = new Set();

    function dfs(r,c,node,path){
        const ch=grid[r][c];
        const nxt=node.ch[ch];
        if(!nxt) return;
        path += ch;
        if(nxt.end) found.add(path);
        vis[r][c]=true;
        for(let dr=-1;dr<=1;dr++) for(let dc=-1;dc<=1;dc++){
            if(dr===0&&dc===0) continue;
            const nr=r+dr, nc=c+dc;
            if(nr>=0&&nr<R&&nc>=0&&nc<C&&!vis[nr][nc]) dfs(nr,nc,nxt,path);
        }
        vis[r][c]=false;
    }

    for(let r=0;r<R;r++) for(let c=0;c<C;c++) dfs(r,c,root,"");

    for(const w of [...found].sort()) console.log(w);
}

main();

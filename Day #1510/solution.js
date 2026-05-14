// Three stacks sharing ONE backing array of nodes (value, prevIndex) + free list.
// Three head pointers index into the single shared list. O(1) push/pop, O(n) space.

class ThreeStacks {
    constructor() {
        this.val = [];
        this.prev = [];
        this.head = [-1, -1, -1];
        this.freeHead = -1;
    }

    _alloc(v, p) {
        let idx;
        if (this.freeHead !== -1) {
            idx = this.freeHead;
            this.freeHead = this.prev[idx];
            this.val[idx] = v;
            this.prev[idx] = p;
        } else {
            idx = this.val.length;
            this.val.push(v);
            this.prev.push(p);
        }
        return idx;
    }

    push(item, s) {
        this.head[s] = this._alloc(item, this.head[s]);
    }

    pop(s) {
        const idx = this.head[s];
        const v = this.val[idx];
        this.head[s] = this.prev[idx];
        this.prev[idx] = this.freeHead;
        this.freeHead = idx;
        return v;
    }
}

const st = new ThreeStacks();
st.push(1, 0); st.push(2, 0);
st.push(3, 1);
st.push(4, 2); st.push(5, 2);
console.log([st.pop(0), st.pop(2), st.pop(1), st.pop(0)].join(" "));

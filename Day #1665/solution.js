// Brick wall: prefix-sum edge positions per row, max edge frequency => fewest cuts = rows - maxEdges. O(total bricks) time/space.
function main(){
  const wall = [[3,5,1,1],[2,3,3,2],[5,5],[4,4,2],[1,3,3,3],[1,1,6,1,1]];
  const freq = new Map();
  let best = 0;
  for(const row of wall){
    let s = 0;
    for(let i = 0; i + 1 < row.length; i++){
      s += row[i];
      const c = (freq.get(s) || 0) + 1;
      freq.set(s, c);
      if(c > best) best = c;
    }
  }
  console.log(wall.length - best);
}
main();

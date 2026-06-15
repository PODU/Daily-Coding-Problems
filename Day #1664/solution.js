// Apply permutation: result[P[i]] = array[i]. O(n) time, O(n) space.
function main(){
  const arr = ["a", "b", "c"];
  const P = [2, 1, 0];
  const res = new Array(arr.length);
  for(let i = 0; i < arr.length; i++) res[P[i]] = arr[i];
  console.log("[" + res.join(", ") + "]");
}
main();

function moveZeros(arr) {
  for (let i=0; i<arr.length; i++) {
    arr.push(arr.splice(arr.indexOf(0),1)[0])
  }
  return arr;
}

console.log(moveZeros([1,2,0,1,0,1,0,3,0,1])); 
console.log(moveZeros([false,1,0,1,2,0,1,3,"a"]));

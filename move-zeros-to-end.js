function moveZeros(arr) {
  for (let i=0; i<arr.length; i++) {
    arr.push(arr.splice(arr.indexOf(0),1)[0])
  }
  return arr;
}

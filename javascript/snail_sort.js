function snail(array) {
  if (array.flat().length === 0) return [];

  const flat = [];
  let y = 0;
  let x = 0;

  const numSteps = Math.pow(array.length, 2);

  let turns = 0;
  let counter = 0;
  let flag = false;
  let stepSize = array.length - 1;

  for (step = 1; step <= numSteps; step++) {
    // console.log(`x: ${x} y: ${y} val: ${array[y][x]}`);
    // flat.push(array[y][x]);
    console.log(`${step} turns: ${turns}, stepsize: ${stepSize}`);
    switch (turns % 4) {
      case 0:
        x += 1;
        break;
      case 1:
        y += 1;
        break;
      case 2:
        x -= 1;
        break;
      case 3:
        y -= 1;
        break;
    }

    if (step % stepSize !== 0) {
      continue;
    }
    turns++;
    counter++;
    if (counter % 2 === 0 && flag) {
      stepSize--;
      counter = 0;
    }
    if (counter % 3 === 0 && !flag) {
      stepSize--;
      flag = true;
      counter = 0;
    }
    console.log(`${step + 1} turns: ${turns}, stepsize: ${stepSize}`);
  }
  return flat;
}

module.exports = snail;

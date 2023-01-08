function ipsBetween(start, end) {
  const startvalues = start.split(".");
  const endvalues = end.split(".");

  let diff = 0;
  let j = 3;
  for (i = 0; i < 4; i++) {
    const curdiff = +endvalues[i] - +startvalues[i];
    diff += curdiff * Math.pow(2, 8 * j);
    j--;
  }
  return diff;
}
ipsBetween("10.0.0.0", "10.0.0.50");
module.exports = ipsBetween;

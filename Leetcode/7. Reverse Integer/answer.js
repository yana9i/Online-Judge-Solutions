/**
 * @param {number} x
 * @return {number}
 */
var reverse = function (x) {
  if (x < Math.pow(-2, 31) || x > Math.pow(2, 31) - 1)
    return 0
  let minus = 1;
  if (x < 0)
    minus = -1;
  const result = parseInt(Array.from(x.toString()).reverse().join("")) * minus;
  if (result < -2147483648 || result > 2147483647)
    return 0
  return result;
}

console.log(reverse(123));
console.log(reverse(-123));
console.log(reverse(0));
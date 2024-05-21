/**
 * @param {number[]} height
 * @return {number}
 * Brutal Force, Not Working
 */
// var maxArea = function (height) {
//   let maxArea = 0;
//   for (let i in height) {
//     for (let j = i; j < height.length; j++) {
//       if (i === j)
//         continue;
//       const heightVal = height[i] < height[j] ? height[i] : height[j]
//       const lengthVal = j - i;
//       const area = heightVal * lengthVal;
//       if (area > maxArea) {
//         maxArea = area;
//       }
//     }
//   }
//   return maxArea
// };

/**
 * @param {number[]} height
 * @return {number}
 */
var maxArea = function (height) {
  let leftIndex = 0;
  let rightIndex = height.length - 1;
  let maxArea = 0;
  while (leftIndex !== rightIndex) {
    const length = rightIndex - leftIndex;
    const currentHeight = height[leftIndex] < height[rightIndex] ? height[leftIndex] : height[rightIndex];
    const area = length * currentHeight;
    if (area > maxArea)
      maxArea = area;
    if (height[leftIndex] < height[rightIndex]) {
      leftIndex++;
    } else {
      rightIndex--;
    }
  }
  return maxArea;
};




console.log(maxArea([1, 8, 6, 2, 5, 4, 8, 3, 7]));
console.log(maxArea([1, 1]));
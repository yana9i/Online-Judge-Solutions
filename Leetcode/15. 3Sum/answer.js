/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var threeSum = function (nums) {
  nums = nums.sort((a, b) => a - b);
  let result = [];
  for (let i = 0; i < nums.length; i++) {
    if (i > 0 && nums[i] === nums[i - 1])
      continue;
    const target = -nums[i];
    let leftIndex = i + 1;
    let rightIndex = nums.length - 1;
    while (leftIndex < rightIndex) {
      const currentResult = nums[leftIndex] + nums[rightIndex];
      if (currentResult < target) {
        leftIndex++;
      } else if (currentResult > target) {
        rightIndex--;
      } else {
        result.push([nums[i], nums[leftIndex], nums[rightIndex]]);
        rightIndex--;
        while (nums[rightIndex] == nums[rightIndex + 1] && leftIndex < rightIndex)
          rightIndex--
      }
    }
  }
  return result;
};

console.log(threeSum([-1, 0, 1, 2, -1, -4]));
console.log(threeSum([0, 1, 1]));
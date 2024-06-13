/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[][]}
 */
var fourSum = function (nums, target) {
  nums = nums.sort((a, b) => a - b);
  let leftOutIndex = 0;
  let rightOutIndex = nums.length - 1;
  const hashFlags = [];
  const result = [];
  while (leftOutIndex < rightOutIndex) {
    let leftInnerIndex = leftOutIndex + 1;
    let rightInnerIndex = rightOutIndex - 1;
    let lastSum = NaN;
    while (leftInnerIndex < rightInnerIndex) {
      console.log(nums[leftOutIndex], nums[leftInnerIndex], nums[rightInnerIndex], nums[rightOutIndex])
      const sum = nums[leftOutIndex] + nums[leftInnerIndex] + nums[rightInnerIndex] + nums[rightOutIndex];
      if (isNaN(lastSum) || sum > lastSum) {
        lastSum = sum;
      }
      if (sum < target) {
        leftInnerIndex++
      } else if (sum > target) {
        rightInnerIndex--;
      } else {
        const currentArray = [nums[leftOutIndex], nums[leftInnerIndex], nums[rightInnerIndex], nums[rightOutIndex]].sort((a, b) => a - b);
        const hashIndex = currentArray.join("+");
        if (hashFlags[hashIndex]) {
          // rightInnerIndex--;
          leftInnerIndex++;
          continue;
        }
        hashFlags[hashIndex] = 1;
        result.push(currentArray);
        // rightInnerIndex--;
        leftInnerIndex++;
      }
    }
    // leftOutIndex++;
    rightOutIndex--
  }
  return result;
};


// console.log(fourSum([1, 0, -1, 0, -2, 2], 0));
// console.log(fourSum([2, 2, 2, 2, 2], 8))
// console.log(fourSum([-3, -1, 0, 2, 4, 5], 2))
// console.log(fourSum([-2, -1, -1, 1, 1, 2, 2], 0))
console.log(fourSum([-3, -2, -1, 0, 0, 1, 2, 3], 0).toString())
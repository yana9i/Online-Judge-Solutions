/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var threeSumClosest = function (nums, target) {
  nums = nums.sort((a, b) => a - b);
  let globalSum = nums[0] + nums[1] + nums[2];
  for (let i = 0; i < nums.length; i++) {
    let leftIndex = i + 1;
    let rightIndex = nums.length - 1;
    while (leftIndex < rightIndex) {
      const currentSum = nums[i] + nums[leftIndex] + nums[rightIndex];
      if (currentSum === target) {
        return currentSum;
      } else if (currentSum < target) {
        leftIndex++;
      } else {
        rightIndex--;
      }
      if (Math.abs(currentSum - target) < Math.abs(globalSum - target)) {
        globalSum = currentSum
      }
    }
  }
  return globalSum;
};


console.log(threeSumClosest([-1, 2, 1, -4], 1));
console.log(threeSumClosest([0, 0, 0], 1));
console.log(threeSumClosest([1, 1, 1, 0], -100))
console.log(threeSumClosest([-1, -1, 2, 4], 1))
console.log(threeSumClosest([4, 0, 5, -5, 3, 3, 0, -4, -5], -2))
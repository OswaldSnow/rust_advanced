fn main() {
    /*
    给你一个有序数组 nums ，请你 原地 删除重复出现的元素，
    使得出现次数超过两次的元素只出现两次 ，返回删除后数组的新长度。
    不要使用额外的数组空间，你必须在 原地 修改输入数组 并在使用 O(1) 额外空间的条件下完成。

    示例1
    输入：nums = [1,1,1,2,2,3]
    输出：5, nums = [1,1,2,2,3]
    解释：函数应返回新长度 length = 5, 并且原数组的前五个元素被修改为 1, 1, 2, 2, 3。
    不需要考虑数组中超出新长度后面的元素。

    示例2
    输入：nums = [0,0,1,1,1,1,2,3,3]
    输出：7, nums = [0,0,1,1,2,3,3]
    解释：函数应返回新长度 length = 7, 并且原数组的前七个元素被修改为 0, 0, 1, 1, 2, 3, 3。
    不需要考虑数组中超出新长度后面的元素。
     */
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    // expect result nums is [1, 1, 2, 2, 3, _]
    let k = remove_duplicates_orderly(&mut nums);
    // expect k is 5
    assert_eq!(k, 5);
    assert_eq!(nums, vec![1, 1, 2, 2, 3, 3]);
}

fn remove_duplicates_orderly(nums: &mut Vec<i32>) -> i32 {
    let mut k = 1usize;
    for i in 1..nums.len() {
        if i + 1 == nums.len() || nums[i - 1] != nums[i + 1] {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as _
}

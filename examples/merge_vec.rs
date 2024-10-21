fn main() {
    /*
    给你两个按 非递减顺序 排列的整数数组 nums1 和 nums2
    另有两个整数 m 和 n ，分别表示 nums1 和 nums2 中的元素数目。
    请你 合并 nums2 到 nums1 中，使合并后的数组同样按 非递减顺序 排列。
    注意：最终，合并后数组不应由函数返回，而是存储在数组 nums1 中。
    为了应对这种情况，nums1 的初始长度为 m + n，
    其中前 m 个元素表示应合并的元素，后 n 个元素为 0 ，应忽略。nums2 的长度为 n 。

    示例1
    输入：nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
    输出：[1,2,2,3,5,6]
    解释：需要合并 [1,2,3] 和 [2,5,6] 。
    合并结果是 [1,2,2,3,5,6] ，其中斜体加粗标注的为 nums1 中的元素。

    示例2
    输入：nums1 = [1], m = 1, nums2 = [], n = 0
    输出：[1]
    解释：需要合并 [1] 和 [] 。
    合并结果是 [1] 。
     */
    let mut nums1 = vec![1, 3, 5, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 4, 6];
    let n = 3;

    merge(&mut nums1, m, &mut nums2, n);
    // expect nums1 is [1, 2, 3, 4, 5, 6]

    assert_eq!(nums1, [1, 2, 3, 4, 5, 6]);
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let m = m as usize;
    let n = n as usize;
    if nums1.len() >= m + n {
        for i in 0..n {
            nums1[i + m] = nums2[i]
        }
    }
    nums1.sort();
}

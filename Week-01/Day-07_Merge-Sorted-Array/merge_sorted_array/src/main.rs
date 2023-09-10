/*
could not find a way to send arrays of different size as input parameter to the function
*/

fn merge_sorted_array(nums1: &[i32], m: i32, nums2: &[i32], n: u16) -> Vec<i32> {
    // 10^9 = 1_000_000_000
    // i32 = +-2^63 = +-2_147_483_648
    println!("params: {:?}, {}, {:?}, {}", nums1, m, nums2, n);

    let mut combined_nums = Vec::new();
    for i in 0..=m-1 {
        println!("element in nums1: {:?}", nums1[i as usize]);
        combined_nums.push(nums1[i as usize]);
    }

    for j in 0..=n-1 {
        println!("element in nums2: {:?}", nums2[j as usize]);
        combined_nums.push(nums2[j as usize]);
    }

    combined_nums.sort();
    combined_nums
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(merge_sorted_array(&[1,2,3,0,0,0], 3, &[2,5,6], 3), [1,2,2,3,5,6]);
    }

}

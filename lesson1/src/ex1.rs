// Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
// Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
//             let sub_arr = [6,8,10];

fn is_sub_array(arr1: &[i32], arr2: &[i32]) -> bool {
    let arr1_length = arr1.len();
    let arr2_length = arr2.len();

    let mut i = 0;
    let mut j = 0;

    while i < arr1_length && j < arr2_length {
        if arr1[i] == arr2[j] {
            i = i + 1;
            j = j + 1;

            if j == arr2_length {
                return true;
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }

    return false;
}

fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];

    if is_sub_array(&org_arr, &sub_arr) {
        print!("YES")
    } else {
        print!("NO")
    }
}

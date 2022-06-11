// Cho 1 chuỗi str Slice như dưới đây. Nhập 1 từ bất kỳ từ bàn phím, in ra số lượng từ này xuất hiện trong chuỗi đã cho.
// Nâng cao hơn : Tìm kiếm không phân biêt chữ hoa thường, theo dạng regex.
use std::io::{self, BufRead};
fn count_occurences_character(source: &str, char: &str) -> usize {
    return source.trim().matches(char.trim()).count();
}

fn main() {
    let stdin = io::stdin();
    let mut source = String::new();
    println!("Enter your string:");
    stdin.lock().read_line(&mut source).unwrap();
    let mut char_input = String::new();
    println!("Enter your character want to find occurencies:");
    stdin.lock().read_line(&mut char_input).unwrap();
    print!(
        "Found: {} ocurrencies in {}",
        count_occurences_character(&source, &char_input),
       &source
    );
}

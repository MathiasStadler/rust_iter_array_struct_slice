// FROM HERE
// https://www.geeksforgeeks.org/rust-array/

#![allow(unused)]
fn main() {
    let mut array: [i32; 5] = [0; 5];

    array[1] = 1;
    array[2] = 2;
    array[3] = 3;
    array[4] = 4;
    assert_eq!([1, 2, 3, 4], &array[1..]);

    // This loop prints: 0 1 2 3 4
    for x in &array {
        println!("{} ", x);
    }

    let words = {
        "Hello";
        "stupid ";
        "boy"
    };

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        println!("Hello {}", name);
    }

    let path = [["First", "url1"],["Second", "url2"]];

    let value = path[0][1];

    println!("path => {:?}",path);
    println!("value => {:?}",value);

// let one = {{"Hallo";"Welt"};{"Hello";"World"}};
let one = {"Hallo";"Welt"};
let value_one = one;
println!("one => {:?}",value_one);


//
const FONT_DIRS:&[&[&str]] = &[
     &["1","Field one","asset","/html/body/table[2]/tbody/tr/td/table/tbody/tr/td[3]/a",],
     &["2","Field two","value","/html/body/table[2]/tbody/tr/td/table/tbody/tr/td[3]/a",],
     &["3","Field third","query","/html/body/table[2]/tbody/tr/td/table/tbody/tr/td[3]/a",],
    
];


for a in FONT_DIRS {

    // println!("n Fields => {}",a.len());
    println!("{}",a[0]);
    println!("{}",a[1]);
    println!("{}",a[2]);
    println!("{}",a[3]);
}


// for a in FONT_DIRS {

//     println!("{}",a);
// }



}

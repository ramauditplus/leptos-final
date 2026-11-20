fn main() {
    println!("Hello, world!");

    let numbers = vec![10, 20, 30, 40, 50];
    let values = vec!["10", "20", "abc", "30", "xyz", "40"];

    let flat_numbers = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    // for (i, number) in numbers.into_iter().enumerate(){
    //     if number > 20{
    //         let double = number * 2;
    //         println!("{i} - {double}")
    //     }
    // }

    // for number in numbers.iter_mut(){
    //     *number *= 2;
    //     println!("{number}")
    // }

    // for (i, number) in numbers.iter_mut().enumerate() {
    //     if i % 2 == 0 {
    //         *number *= 2
    //     } else {
    //         *number += 5
    //     }
    // }

    // println!("{numbers:?}")

    // let result: Vec<i32> = numbers
    //     .iter()
    //     .filter(|number| **number > 15)
    //     .map(|number| *number * 2)
    //     .collect();
    // println!("{result:?}");
    // println!("{numbers:?}");

    // let result = values
    //     .iter()
    //     .filter_map(|s| s.parse::<i32>().ok().map(|x| x * 2))
    //     .collect::<Vec<i32>>();
    // println!("{result:?}");
    // println!("{values:?}");

    // let result: Vec<i32> = flat_numbers
    //     .into_iter()
    //     .flat_map(|v| v)
    //     .collect();
    // println!("{result:?}");
}

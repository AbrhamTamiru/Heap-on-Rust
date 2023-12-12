// Name Abrham Tamiru
// Class (CECS 342-07)
// Project Name (Prog 5 – Rust Heap sort)
// Due Date (12/07/1997)
//
// I certify that this program is my own original work. I did not copy any part of this program from
// any other source. I further certify that I typed each and every line of code in this program.

fn max_heapify<T: Ord>(arr: &mut Vec<T>, n: usize, i: usize)
{
    // compare the root with its children 
    // find the larget 
    let mut largest=i;
    let  left_child= 2*i+1;
    let  right_child=2*i+2;
    if left_child < n && arr[left_child]> arr[largest]
    {
        largest=left_child;
    }
    if right_child < n && arr[right_child]>arr[largest] 
    {
        largest=right_child;
    }
    if largest!=i {
        arr.swap(i,largest);// make the largest the root
        max_heapify(arr, n, largest);
    }
}

fn build_heap<T: Ord>(arr: &mut Vec<T>)
{
    // using a loop to built a tree
    let n= arr.len();
    for i in (0..n/2).rev()
    {
        max_heapify(arr,n,i);
    }
}
fn heap_sort<T: Ord + std::fmt::Debug>(arr: &mut Vec<T>)
{
    let n=arr.len();
    build_heap(arr);
    for i in (1..n).rev()
    {
        arr.swap(0,i); // swap the root with the last element
        max_heapify(arr, i,0);
        println!("{:?}",arr) // allow to print the array using debug
    }
}
use std::io;  // use to ask input
fn main()
{
    let mut b =vec![20,34,64,17,76,15,12,98,31,78,56,37,68,74,18,62,86,50,77,70,85,99,46,84,93,21,16,66,49,57,35];
    println!("Orginal list");
    println!("{:?}",b);
    println!("orginal list heap tree");
    println!("                                                  {:?}",b[0]);
    println!("                                                   |");
    println!("                             {:?}--------------------^--------------------------{:?}",b[1],b[2]);
    println!("                             |                                                 |");
    println!("               {:?}------------^------------{:?}                     {:?}------------^-------------{:?}",b[3],b[4],b[5],b[6]);
    println!("               |                           |                      |                           |");
    println!("        {:?}-----^----{:?}               {:?}----^----{:?}           {:?}---^---{:?}                  {:?}--^-----{:?}",b[7],b[8],b[9],b[10],b[11],b[12],b[13],b[14]);
    println!("        |            |              |            |            |       |                   |           |");
    println!("     {:?}-^-{:?}      {:?}-^-{:?}       {:?}-^-{:?}       {:?}-^-{:?}     {:?}-^-{:?}  {:?}-^-{:?}             {:?}-^-{:?}     {:?}-^-{:?}",b[15],b[16],b[17],b[18],b[19],b[20],b[21],b[22],b[23],b[24],b[25],b[26],b[27],b[28],b[29],b[30]);
    println!();
    println!("Enter any key to continue :");
    let mut line=String::new();// store user input 
    // if a user press any key it will continue
    if let b1= std::io::stdin().read_line(&mut line).unwrap() {
        heap_sort(&mut b);
        println!();
        println!("Here is the sorted tree:");
        println!("                                                  {:?}",b[0]);
        println!("                                                   |");
        println!("                             {:?}--------------------^--------------------------{:?}",b[1],b[2]);
        println!("                             |                                                 |");
        println!("               {:?}------------^------------{:?}                     {:?}------------^-------------{:?}",b[3],b[4],b[5],b[6]);
        println!("               |                           |                      |                           |");
        println!("        {:?}-----^----{:?}               {:?}----^----{:?}           {:?}---^---{:?}                  {:?}--^-----{:?}",b[7],b[8],b[9],b[10],b[11],b[12],b[13],b[14]);
        println!("        |            |              |            |            |       |                   |           |");
        println!("     {:?}-^-{:?}      {:?}-^-{:?}       {:?}-^-{:?}       {:?}-^-{:?}     {:?}-^-{:?}  {:?}-^-{:?}             {:?}-^-{:?}     {:?}-^-{:?}",b[15],b[16],b[17],b[18],b[19],b[20],b[21],b[22],b[23],b[24],b[25],b[26],b[27],b[28],b[29],b[30]);
        println!();
        println!("Here is the sorted list:");
        println!("{:?}",b);
        }
}
























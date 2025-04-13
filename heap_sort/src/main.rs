fn heapify(arr:&mut Vec<i32>,len:usize,i:usize){
println!("{}\n",arr[i]);
}

fn heap_sort(arr:&mut Vec<i32>,len:usize){

for i in (0..=len).rev(){
    heapify( arr,len,i);
}
}
fn main() {
    let mut arr=vec![3,5,88,99,33,1,33,66,777,7,4,2];
    let mut length=arr.len();
    let len:usize=if length > 1 { (length / 2) - 1 } else { 0 };
    heap_sort(&mut arr,len);
}

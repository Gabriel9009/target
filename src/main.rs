fn main() {
    let arr : [i32; 4] =[3 ,2, 4, 3];
    let target : i32 = 6;
    let mut count = 1;
    for i in 0..arr.len() - 1{
        for j in count..arr.len(){
            if arr[i] + arr[j] == target{
                println!("{} {} ", i, j);
                count+= 1;
            }else{
            }
        }
    }
}

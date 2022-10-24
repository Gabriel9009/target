fn main() {
    let sx : [i32; 4] =[3 ,2, 4, 3];
    let target : i32 = 6;
    let mut count = 1;
    for i in 0..sx.len() - 1{
        for j in count..sx.len(){
            if sx[i] + sx[j] == target{
                println!("{} {} ", i, j);
                count+= 1;
            }else{
            }
        }
    }
}

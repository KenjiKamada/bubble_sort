use rand::Rng;

fn main() {

    let mut x: [i32; 101]=[0;101]; //乱数
    let mut y: [i32; 100]=[0;100]; // ソート結果

    // -1000 以上 1000 以下のランダムな整数を100個用意する。
    for n in 1..x.len(){
        x[n] = rand::thread_rng().gen_range(-(1000+1), 1000+1);
    }
    bubble_sort(&mut x, &mut y);
    
    //let mut tmp: [i32;2]=[0;2];
  //  let mut tmp = [0,0];
   
    // バブルソート
    // for n in 1.. x.len(){
    //     for m in 2.. x.len(){
    //         tmp[0]=x[m-1];  tmp[1]=x[m];
    //                if x[m] < x[m-1]{
    //             x[m-1] = tmp[1];
    //             x[m] = tmp[0];
    //         }
    //     } 
    // }

 

     // copy x -> y
    // for n in 0.. y.len(){
    //     y[n] = x[n+1];
    //   }

    // 確認  yにソート結果
    // 全部"True！"なら良い。
    for n in 0.. y.len()-1{
      //  println!("n={}, x={}, y={}", n,x[n], y[n]);

        if y[n] <= y[n+1] {
            println!("True!")
        }else{
            println!("False...")
        }
    }

}
fn bubble_sort(x: &mut [i32], y: &mut [i32]){

  //  println!("x={}",x[10]);
  let mut tmp = [0,0];

    // バブルソート
    for n in 1.. x.len(){
        for m in 2.. x.len(){
            tmp[0]=x[m-1];  tmp[1]=x[m];
                   if x[m] < x[m-1]{
                x[m-1] = tmp[1];
                x[m] = tmp[0];
            }
        } 
    }

    // copy x -> y
    for n in 0.. y.len(){
    y[n] = x[n+1];
    }
}




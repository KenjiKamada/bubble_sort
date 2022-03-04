use rand::Rng;

fn main() {

    const N:usize = 100; //N個の整数数列

    let mut x: [i32; N]=[0;N]; //乱数
    let mut y: [i32; N]=[0;N]; // ソート結果

    // -1000 以上 1000 以下のランダムな整数を100個用意する。
    for n in 1..x.len(){
        x[n] = rand::thread_rng().gen_range(-1000, 1000+1);
    }
 
       bubble_sort(&mut x, &mut y); // xからyにソートする関数 
 
    // 確認  yにソート結果
    // 全部"True！"なら良い。
    for n in 0.. y.len()-1{
        println!("sort前={}, sort結果={}", x[n], y[n]);
        assert!(y[n] <= y[n+1]);
    }
}
/*  xからyにソートする関数 */
fn bubble_sort(x: &mut [i32], y: &mut [i32]){

  //  println!("x={}",x[10]);
  let mut tmp = [0,0];

    // copy x -> y
    for n in 0.. y.len(){
        y[n] = x[n];
    }

    // バブルソート
    for _n in {0.. x.len()-2}.rev(){
        for m in 1.. x.len(){
            tmp[0]=y[m-1];  
            tmp[1]=y[m];
            if y[m] < y[m-1]{
                y[m-1] = tmp[1];
                y[m] = tmp[0];
            }
        } 
    }
}

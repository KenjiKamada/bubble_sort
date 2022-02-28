use rand::Rng;

fn main() {

    let a = 1000;

    let mut x: [i32; 101]=[0;101];
    let mut y: [i32; 100]=[0;100];

    // -1000 以上 1000 以下のランダムな整数を100個用意する。
    for n in 1..x.len(){
        x[n] = rand::thread_rng().gen_range(-(a+1), a+1);
    }

    
    //let mut tmp: [i32;2]=[0;2];
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

       // 確認
    for n in 0.. y.len(){
        println!("n={}, x={}, y={}", n,x[n], y[n]);
    }

}




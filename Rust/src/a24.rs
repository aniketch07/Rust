fn main() {
    let data = vec![1, 2, 3, 4, 5];



    let newdata: Vec<_> = data.iter().map(|num| num * 3).collect();
  

    let filterdata: Vec<_> = newdata.iter().filter(|&num|num>&10).collect();

    for d in filterdata{
        println!("data:{}",d);
    }


    let data2: Vec<_> = vec![1, 2, 3, 4, 5].iter().map(|num|num*3).filter(|num|num>&10).collect();
    for d in data2{
        println!("data:{}",d);
    }

}


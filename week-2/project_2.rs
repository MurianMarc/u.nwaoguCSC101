fn main (){
    //define the quantity of the products
    let tq:f64 = 2.0;
    let mq:f64 = 1.0;
    let hq:f64 = 3.0;
    let dq:f64 = 3.0;
    let aq:f64 = 1.0;

    //define the amount of sales of the products
    let ta:f64 = 450_000.0;
    let ma:f64 = 1_500_000.0;
    let ha:f64 = 750_000.0;
    let da:f64 = 2_850_000.0;
    let aa:f64 = 250_000.0;

    //calculate the sum and print

let sum = (tq * ta) + (mq * ma) + (hq *ha) + (dq * da) + (aq + aa);

println! ("your sum of sales is {}", sum);

//calculate the average and print

let avg = sum/(tq + mq + hq + dq + aq);

println! ("And your average sales is {}", avg);

}
fn main(){ /*I begin by assigning the various products to their quantities, then I create separate variables representing their 
	respective prices.*/
	let toshiba: f64 = 2.0;
	let t_price: f64 = 450_000.00;

    let mac: f64 = 1.0;
    let m_price: f64 = 1_500_000.00;

    let hp: f64 = 3.0;
    let hp_price: f64 = 750_000.00;

    let dell: f64 = 3.0;
    let d_price: f64 = 2_850_000.00;

    let acer: f64 = 1.0;
    let a_price: f64 = 250_000.00;

    let sum_quantity: f64 = (toshiba + mac + hp + dell + acer).into();//I sum up the number of products(computers) in stock.
    let sum_price: f64 = (t_price * toshiba) + (m_price * mac) + (hp_price * hp) + (d_price * dell) + (a_price * acer);
    //In the line above, I sum up their respective prices, in accordances to how many of each computer is in stock.
    let average: f64 = sum_price/sum_quantity; //I calculate the average by dividing the total value by the number of items in stock.
    println!("The total number of computers in stock is: {}, with a total value of N{}, and the average cost of each one is: N{}.", sum_quantity, sum_price, average);
}


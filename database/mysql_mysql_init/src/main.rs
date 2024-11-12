use dotenv;
use mysql::{prelude::*,
            *};
use std::{error::Error,
          result::Result};

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id:  i32,
    amount:       i32,
    account_name: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let url: String = dotenv::var("DATABASE_URL").unwrap();
    let pool: Pool = Pool::new(url.as_str())?;
    let mut conn: PooledConn = pool.get_conn()?;

    conn.query_drop(r"
                    CREATE TABLE IF NOT EXISTS payment (
                        customer_id int not null,
                        amount int not null,
                        account_name text
                    )
                    ")?;

    let payments: Vec<Payment> =
        vec![Payment { customer_id:  1,
                       amount:       2,
                       account_name: None, },
             Payment { customer_id:  3,
                       amount:       4,
                       account_name: Some("foo".into()), },
             Payment { customer_id:  5,
                       amount:       6,
                       account_name: None, },
             Payment { customer_id:  7,
                       amount:       8,
                       account_name: None, },
             Payment { customer_id:  9,
                       amount:       10,
                       account_name: Some("bar".into()), },];

    conn.exec_batch(r"
                    INSERT INTO payment (customer_id, amount, account_name)
                    VALUES (:customer_id, :amount, :account_name)
                    ",
                    payments.iter()
                            .map(|p: &Payment| {
                                params! {
                                    "customer_id" => p.customer_id,
                                    "amount" => p.amount,
                                    "account_name" => &p.account_name,
                                }
                            }))?;

    let selected_payments: Vec<Payment> =
        conn.query_map(r"
                                           SELECT customer_id, amount, account_name FROM payment
                                           ",
                       |(customer_id, amount, account_name)| Payment { customer_id,
                                                                       amount,
                                                                       account_name })?;

    for row in selected_payments {
        println!("{:?}", row);
    }
    println!("Yay!");

    Ok(())
}

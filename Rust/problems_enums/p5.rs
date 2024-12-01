// Problem: Create an enum PaymentMethod with the variants:

// CreditCard { number: String, expiry: String }
// PayPal { email: String }
// Cash { amount: f32 }
// Write a function process_payment that processes each payment method differently.
//  For example, print the card number for CreditCard, print the email for PayPal, and print the amount for Cash.

enum PaymentMethod{
    CreditCard { number: String, expiry: String },
    PayPal { email: String },
    Cash { amount: f32 },

}

fn process_payment(pay_method:PaymentMethod){
    match pay_method{
        PaymentMethod::CreditCard{number,expiry}=>println!("card number:{} expiry:{}",number,expiry),
        PaymentMethod::PayPal{email}=>println!("email:{}",email),
        PaymentMethod::Cash{amount}=>println!("amount:{}",amount),
  }
}

fn main(){
    let cc = PaymentMethod::CreditCard {
        number: "10".to_string(),
        expiry: "22/12/2022".to_string(),
    };

    let pp = PaymentMethod::PayPal {
        email: "caniket1307@gmail.com".to_string(),
    };

    let c = PaymentMethod::Cash {
        amount: 199.0,
    };

    // Process payments
    process_payment(cc);

}
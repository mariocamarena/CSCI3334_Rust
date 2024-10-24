mod bank_account;

use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(100.0);
    println!("starting balance: {}", account.balance());
    
    account.deposit(50.0);
    println!("balance after deposit: {}", account.balance());
    
    account.withdraw(30.0);
    println!("balance after withdrawal: {}", account.balance());
}
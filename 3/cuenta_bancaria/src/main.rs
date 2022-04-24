use std::{thread, sync::{Arc, Mutex}};

struct Account(i32);

impl Account {
    fn deposit(&mut self, amount: i32) {
        println!("op: deposit {}, available funds: {:?}", amount, self.0);
        self.0 += amount;
    }
    
    fn withdraw(&mut self, amount: i32) -> Result<(),String>{
        println!("op: withdraw {}, available funds: {}", amount, self.0);
        if self.0 >= amount {
            self.0 -= amount;
            Ok(())
        } else {
            Err(String::from("Error: Insufficient funds."))
        }
    }
    
    fn balance(&self) -> i32 {
        self.0
    }
}

//static mut ACCOUNT: Account = Account(Arc::newMutex::new(0)>);

fn main() {

    let account = Arc::new(Mutex::new(Account(0)));

    let account_1 = Arc::clone(&account);
    let account_2 = Arc::clone(&account);
    let account_3 = Arc::clone(&account);
    let account_4 = Arc::clone(&account);
    
    let customer1_handle = thread::spawn(move || {
        let mut funds = account_1.lock().unwrap();
        (*funds).deposit(40);
    });
    

    let customer2_handle = thread::spawn(move || {
        let mut funds = account_2.lock().unwrap();
        match (*funds).withdraw(30) {
            Ok(()) => (),
            Err(error) => println!("{}",error),
        }
    });
    
    let customer3_handle = thread::spawn(move || {
        let mut funds = account_3.lock().unwrap();
        (*funds).deposit(60);
    });
    
    let customer4_handle = thread::spawn(move || {
        let mut funds = account_4.lock().unwrap();
        match (*funds).withdraw(70) {
            Ok(()) => (),
            Err(error) => println!("{}",error),
        }
    });
    
    let handles = vec![
    customer1_handle,
    customer2_handle,
    customer3_handle,
    customer4_handle,
    ];
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let funds = account.lock().unwrap();
    let savings = (*funds).balance();
    
    println!("Balance: {:?}", savings);
}

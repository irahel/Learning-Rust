//Struct is by default allocated on stack
struct Holder{
    name: String,
    byname: String,
}

struct BankAccount{
    holder: Holder,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        self.balance -= amount;
    }
}

fn main(){
    let mut account = BankAccount{
        holder: Holder{
            name: String::from("Rahel"),
            byname: String::from("Martim"),
        },
        balance: 100.0,
    };

    println!("Account name: {}", account.holder.name);
    println!("Account byname: {}", account.holder.byname);
    println!("Account balance: {}", account.balance);

    account.withdraw(50.0);
    println!("Account balance: {}", account.balance);
}
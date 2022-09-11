struct User {
    name: String,
    balance: (f32, String),
}

impl User {
    fn print_user_detail(&self) {
        println!("Name: {}", self.name);
        println!("Balance: {} {} \n", self.balance.0, self.balance.1);
    }
}

fn main() {
    let mut compounding_count = 5;
    let interest = 5.0;
    let mut user = User {
        name: "Eskaine".to_owned(),
        balance: (1000.0, "USD".to_owned()),
    };

    accrue_interest(&mut user, interest);

    while compounding_count > 0 {
        accrue_interest(&mut user, interest);
        compounding_count -= 1;
    }
}

fn accrue_interest(user: &mut User, interest_percentage: f32) {
    user.balance.0 += user.balance.0 * (interest_percentage / 100.0);
    user.print_user_detail();
}

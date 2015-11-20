// static mut global_last: &'static str = "";

fn main() {
    // global_last = "hello";
    let mut last : &String;

    // block 1
    {
        let user = "user".to_string();
        // print(user)
        {
            let msg = &user;
            println!("{}", msg);
            last = msg;
        }
        // print(user)
        {
            let msg = &user;
            println!("{}", msg);
            last = msg;
        }
    }
    // block 2
    {
        let pass = "pass".to_string();
        // print(NULL)
        {
            println!("{}", last);
        }
        // print(pass)
        {
            let msg = &pass;
            println!("{}", msg);
            last = msg;
        }
    }
}

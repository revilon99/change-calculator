use rand::Rng;

const COINS: [i32; 11] = [2000, 1000, 500, 200, 100, 50, 20, 10, 5, 2, 1];

fn main() {
    let owed: i32 = rand::thread_rng().gen_range(1000..1500) as i32;
    let owed_pound: f32 = (owed as f32) / 100.0;
    println!("Amount Owed: {:.2}", owed_pound);

    println!("Amount Given:");
    let mut input_text = String::new();
    let _ = std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<f32>() {
        Ok(i) => {
            let given: i32 = (i * 100.0).floor() as i32;
            let change: i32 = given - owed;

            if change < 0 {
                println!("Not enough given!");
                return;
            }

            let mut change_arr: [i32; COINS.len()] = [0; COINS.len()];
            calculate_change(change, &mut change_arr);

            println!("You are owed: {:.2}", (change as f32) / 100.0);
            for i in 0..COINS.len() {
                if change_arr[i] < 1 {
                    continue;
                };
                println!("{}x £{:.2}", change_arr[i], (COINS[i] as f32) / 100.0);
            }
        }
        Err(..) => println!("this was not an acceptable input: {}", trimmed),
    };
}

fn calculate_change(change: i32, change_arr: &mut [i32]) {
    let mut c = change.clone();
    for i in 0..COINS.len() - 1 {
        let value = COINS[i];

        while c >= value {
            change_arr[i] += 1;
            c -= value;
        }
    }
    return;
}

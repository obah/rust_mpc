//addictive secret sharing
use rand::{thread_rng, Rng};

const PRIME: u32 = 127;

fn calc_mod(v: i32) -> i32 {
    let prime = PRIME as i32;
    let new_v = if v < 0 { v + prime } else { v };

    new_v % prime
}

fn share(x: i32, n: i32) -> Vec<i32> {
    let mut shares = Vec::new();
    let mut sum = 0;
    let prime = PRIME as i32;

    for _ in 0..n - 1 {
        let mut rng = thread_rng();

        let sign = if rng.gen_bool(0.5) { 1 } else { -1 };

        let rand_num = rng.gen_range(0..prime);

        let r = sign * rand_num;

        sum = calc_mod(sum + r);
        shares.push(r);
    }

    shares.push(calc_mod(x - sum));

    shares
}

fn open(shares: &[i32]) -> i32 {
    shares.iter().fold(0, |sum, &share| calc_mod(sum + share))
}

fn sum_received_shares(received_shares: &[i32]) -> i32 {
    let sum_shares = received_shares
        .iter()
        .fold(0, |sum, &share| calc_mod(sum + share));

    sum_shares
}

pub fn addictive_sharing() {
    // Each person's inputs
    let alice_input = 10;
    let bob_input = 13;
    let chris_input = 22;

    let alice_shares = share(alice_input, 3);
    let bob_shares = share(bob_input, 3);
    let chris_shares = share(chris_input, 3);

    println!(
        "Input from: Alice = {:?}, Bob = {:?}, Chris = {:?}",
        alice_shares, bob_shares, chris_shares
    );

    println!(
        "Input openned from: Alice = {:?}, Bob = {:?}, Chris = {:?}",
        open(&alice_shares),
        open(&bob_shares),
        open(&chris_shares)
    );

    // Each person sends their nth share to the nth party
    let received_alice_shares = [alice_shares[0], bob_shares[0], chris_shares[0]];
    let received_bob_shares = [alice_shares[1], bob_shares[1], chris_shares[1]];
    let received_chris_shares = [alice_shares[2], bob_shares[2], chris_shares[2]];

    // Each person computes the sum of their received shares and broadcasts this value to the other parties.

    // These sums of received shares, neither individually nor in pairs, reveal nothing about any person's inputs.

    let sum_received_alice_shares = sum_received_shares(&received_alice_shares);
    let sum_received_bob_shares = sum_received_shares(&received_bob_shares);
    let sum_received_chris_shares = sum_received_shares(&received_chris_shares);

    println!(
        "Total recieved shares: Alice = {:?}, Bob = {:?}, Chris = {:?}",
        calc_mod(sum_received_alice_shares),
        calc_mod(sum_received_bob_shares),
        calc_mod(sum_received_chris_shares)
    );

    // When each person adds these sums with each other, they get the sum over all their original inputs!
    let total_input =
        calc_mod(sum_received_alice_shares + sum_received_bob_shares + sum_received_chris_shares);

    println!("Total sum: {}", total_input);
}

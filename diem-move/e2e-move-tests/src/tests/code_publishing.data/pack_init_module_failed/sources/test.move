module 0xcafe::test {
    use diem_framework::coin::{Self, Coin};
    use diem_framework::diem_coin::DiemCoin;
    use std::signer::address_of;

    struct State has key {
        important_value: u64,
        coins: Coin<DiemCoin>,
    }

    fun init_module(s: &signer) {
        // Transfer away all the APT from s so there's nothing left to pay for gas.
        // This makes this init_module function fail for sure.
        let balance = coin::balance<DiemCoin>(address_of(s));
        let coins = coin::withdraw<DiemCoin>(s, balance);

        move_to(s, State {
            important_value: get_value(),
            coins,
        })
    }

    fun get_value(): u64 {
        1
    }
}
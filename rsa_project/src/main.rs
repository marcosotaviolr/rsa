use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};

// Função para calcular o inverso modular (usando Euclides estendido)
fn modinv(a: &BigUint, m: &BigUint) -> BigUint {
    let (mut t, mut new_t) = (BigUint::zero(), BigUint::one());
    let (mut r, mut new_r) = (m.clone(), a.clone());

    while new_r != BigUint::zero() {
        let q = &r / &new_r;

        let temp_t = new_t.clone();
        new_t = if &t > &(&q * &new_t) {
            (&t - &q * &new_t) % m
        } else {
            (m - ((&q * &new_t) - &t) % m) % m
        };
        t = temp_t;

        let temp_r = new_r.clone();
        new_r = &r - &q * &new_r;
        r = temp_r;
    }

    t % m
}

// Exponenciação modular (base^exp mod n)
fn mod_exp(base: &BigUint, exp: &BigUint, n: &BigUint) -> BigUint {
    base.modpow(exp, n)
}

fn main() {
    // Primos pequenos (exemplo didático)
    let p = 61u32.to_biguint().unwrap();
    let q = 53u32.to_biguint().unwrap();

    let n = &p * &q; // n = p*q
    let phi = (&p - 1u32) * (&q - 1u32);
    let e = 17u32.to_biguint().unwrap(); // expoente público
    let d = modinv(&e, &phi); // expoente privado

    println!("Chave pública: (n={}, e={})", n, e);
    println!("Chave privada: (n={}, d={})", n, d);

    // Mensagem numérica
    let mensagem = 42u32.to_biguint().unwrap();
    println!("\nMensagem original (número): {}", mensagem);

    let cifrado = mod_exp(&mensagem, &e, &n);
    println!("Cifrado: {}", cifrado);

    let decifrado = mod_exp(&cifrado, &d, &n);
    println!("Decifrado: {}", decifrado);
}
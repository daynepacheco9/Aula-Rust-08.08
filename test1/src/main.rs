const VALOR: u8 = 123;//como criar constantes


fn main() {
    let mut x = 5; 
    // let é uma variavel que pode alterar o valor porém precisa do mut
    println!("o valor de x é: {x}");
    x = 6;
    println!("o valor de x é: {VALOR}");
}

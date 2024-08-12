//const VALOR: u8 = 123;//como criar constantes
fn main() {
    let var: i16 = 5; 
    let var1: i8 = 4;
    let var2: u16 = 15;
    let var3: u128 = 100;
    let var4: u64 = 50;
    let var5: i32 = 29;

    // let é uma variavel que pode alterar o valor porém precisa do mut
    println!("o valor de var é: {var};\nvar1 = {var1};\nvar1 = {var2};\nvar1 = {var3};\nvar1 = {var4};\nvar1 = {var5}\n\n");
    //mudando de tipo
    let change_var5 = format!("{:X}",var5);
    let change_var1 = format!("{:b}", var1);
    println!("o valor de var5 em hex é: {change_var5};\nvar1 em binário é: {change_var1}\n\n");

    //MIN e MAX
    println!("i8: MIN = {}, MAX = {}", std::i8::MIN, std::i8::MAX);
    println!("u8: MIN = {}, MAX = {}", std::u8::MIN, std::u8::MAX);
    println!("i16: MIN = {}, MAX = {}", std::i16::MIN, std::i16::MAX);
    println!("u16: MIN = {}, MAX = {}", std::u16::MIN, std::u16::MAX);
    println!("i32: MIN = {}, MAX = {}", std::i32::MIN, std::i32::MAX);
    println!("u32: MIN = {}, MAX = {}", std::u32::MIN, std::u32::MAX);
    println!("i64: MIN = {}, MAX = {}", std::i64::MIN, std::i64::MAX);
    println!("u64: MIN = {}, MAX = {}", std::u64::MIN, std::u64::MAX);
    println!("i128: MIN = {}, MAX = {}", std::i128::MIN, std::i128::MAX);
    println!("u128: MIN = {}, MAX = {}\n\n", std::u128::MIN, std::u128::MAX);

    //Operações básicas
    let a: i32 = 10;
    let b: i32 = 3;
    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = a / b;
    let rest = a % b;

    println!("{a} + {b} = {sum}");
    println!("{a} - {b} = {difference}");
    println!("{a} * {b} = {product}");
    println!("{a} / {b} = {quotient} (resto = {rest})");

}

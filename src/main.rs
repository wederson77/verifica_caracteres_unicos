fn tem_caracteres_unicos(input: &str)->bool{
    let mut conjunto_de_caracteres = [false; 128];

    for &c in input.as_bytes(){
        let indice = c as usize;
        println!("Caracters: {}, indice: {}", c as char, indice); //Debug
        
    if conjunto_de_caracteres[indice]{
        println!("Caractere duplicado encontrado!"); // Debug
        return false;
    }
    conjunto_de_caracteres[indice] = true;
}


    true
}

fn main() {
    let string_de_teste = "abcde";
    if tem_caracteres_unicos(string_de_teste){
        println!("A string possui todos os caracteres unicos");
    }else{
        println!("A string não possui todos os caracteres unicos")
    }
}

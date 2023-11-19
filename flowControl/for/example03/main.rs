fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // iter- Isso empresta cada elemento da coleção em cada iteração. Deixando assim a coleção intocada e disponível para reutilização após o loop.

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name ),
        }
    }
    
    println!("names: {:?}", names);
}
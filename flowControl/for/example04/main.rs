fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // into_iter- Isso consome a coleção para que a cada iteração sejam fornecidos os dados exatos.
    // Depois que a coleção for consumida, ela não estará mais disponível para reutilização, pois foi “movida” dentro do loop.

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
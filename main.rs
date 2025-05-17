mod product;
mod search;

use product::Product;
use search::ProductIndex;

fn main() {
    let mut index = ProductIndex::new();

    let p1 = Product {
        id: 1,
        name: "Smartphone X".into(),
        brand: "TechCorp".into(),
        category: "Eletrônicos".into(),
    };

    let p2 = Product {
        id: 2,
        name: "Smartphone Y".into(),
        brand: "TechCorp".into(),
        category: "Eletrônicos".into(),
    };

    index.add_product(p1);
    index.add_product(p2);

    if let Some(results) = index.search_by_brand("TechCorp") {
        println!("Produtos da TechCorp:");
        for product in results {
            println!("{:?}", product);
        }
    } else {
        println!("Nenhum produto encontrado.");
    }
}

use megastore_search::product::Product;
use megastore_search::search::ProductIndex;

#[test]
fn test_search_by_name() {
    let mut index = ProductIndex::new();

    let product = Product {
        id: 1,
        name: "Notebook Gamer".into(),
        brand: "PowerTech".into(),
        category: "Informática".into(),
    };

    index.add_product(product.clone());

    let result = index.search_by_name("Notebook Gamer");
    assert!(result.is_some());
    assert_eq!(result.unwrap()[0], product);
}

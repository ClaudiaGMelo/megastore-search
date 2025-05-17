use std::collections::HashMap;
use crate::product::Product;

pub struct ProductIndex {
    name_index: HashMap<String, Vec<Product>>,
    brand_index: HashMap<String, Vec<Product>>,
    category_index: HashMap<String, Vec<Product>>,
}

impl ProductIndex {
    pub fn new() -> Self {
        Self {
            name_index: HashMap::new(),
            brand_index: HashMap::new(),
            category_index: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.name_index.entry(product.name.clone())
            .or_default().push(product.clone());

        self.brand_index.entry(product.brand.clone())
            .or_default().push(product.clone());

        self.category_index.entry(product.category.clone())
            .or_default().push(product);
    }

    pub fn search_by_name(&self, name: &str) -> Option<&Vec<Product>> {
        self.name_index.get(name)
    }

    pub fn search_by_brand(&self, brand: &str) -> Option<&Vec<Product>> {
        self.brand_index.get(brand)
    }

    pub fn search_by_category(&self, category: &str) -> Option<&Vec<Product>> {
        self.category_index.get(category)
    }
}

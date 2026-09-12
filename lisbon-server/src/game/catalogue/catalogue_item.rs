//! Mirrors `net.h4bbo.lisbon.game.catalogue.CatalogueItem`.

use crate::game::catalogue::catalogue_package::CataloguePackage;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::base::item_definition::ItemDefinition;
use crate::game::item::item_manager::ItemManager;
use crate::log::Log;

#[derive(Clone, Debug, serde::Serialize)]
pub struct CatalogueItem {
    sale_code: String,
    order_id: i32,
    is_hidden: bool,
    price: i32,
    definition: Option<ItemDefinition>,
    item_special_id: i32,
    packages: Vec<CataloguePackage>,
    pages: Vec<i32>,
    name: String,
    description: String,
    package_name: String,
    package_description: String,
    is_package: bool,
    id: i32,
}

impl CatalogueItem {
    /// Mirrors the 13-arg `CatalogueItem` constructor with the `pageId` string.
    pub fn new(
        id: i32,
        sale_code: &str,
        page_id: &str,
        order_id: i32,
        price: i32,
        hidden: bool,
        definition_id: i32,
        item_special_id: i32,
        name: &str,
        description: &str,
        is_package: bool,
        package_name: &str,
        package_description: &str,
    ) -> Self {
        let mut pages = Vec::new();

        if !page_id.is_empty() {
            for data in page_id.split(',') {
                pages.push(data.parse::<i32>().unwrap_or(0));
            }
        }

        Self::with_pages(
            id,
            sale_code,
            pages,
            order_id,
            price,
            hidden,
            definition_id,
            item_special_id,
            name,
            description,
            is_package,
            package_name,
            package_description,
        )
    }

    /// Mirrors the 13-arg `CatalogueItem` constructor with the `pages` array.
    pub fn with_pages(
        id: i32,
        sale_code: &str,
        pages: Vec<i32>,
        order_id: i32,
        price: i32,
        hidden: bool,
        definition_id: i32,
        item_special_id: i32,
        name: &str,
        description: &str,
        is_package: bool,
        package_name: &str,
        package_description: &str,
    ) -> Self {
        let definition = ItemManager::get_instance().get_definition(definition_id);

        if definition.is_none() && !is_package {
            Log::get_error_logger().error(format!(
                "Item ({sale_code}) has an invalid definition id: {definition_id}"
            ));
        }

        Self {
            id,
            sale_code: sale_code.to_string(),
            order_id,
            price,
            is_hidden: hidden,
            definition,
            item_special_id,
            name: name.to_string(),
            description: description.to_string(),
            is_package,
            packages: Vec::new(),
            pages,
            package_name: package_name.to_string(),
            package_description: package_description.to_string(),
        }
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> &str {
        if self.is_package {
            return &self.package_name;
        }

        &self.name
    }

    /// Mirrors `getDescription()`.
    pub fn get_description(&self) -> &str {
        if self.is_package {
            return &self.package_description;
        }

        &self.description
    }

    /// Mirrors `getType()`.
    pub fn get_type(&self) -> &'static str {
        if self.is_package {
            return "d";
        }

        if let Some(definition) = &self.definition {
            if definition.has_behaviour(ItemBehaviour::WallItem) {
                return "i";
            }
        }

        "s"
    }

    /// Mirrors `getIcon()`.
    pub fn get_icon(&self) -> String {
        if self.is_package {
            return String::new();
        }

        self.definition
            .as_ref()
            .map(|definition| definition.get_icon(self.item_special_id))
            .unwrap_or_default()
    }

    /// Mirrors `getSize()`.
    pub fn get_size(&self) -> &'static str {
        if self.is_package
            || self
                .definition
                .as_ref()
                .is_some_and(|definition| definition.has_behaviour(ItemBehaviour::WallItem))
        {
            return "";
        }

        "0"
    }

    /// Mirrors `getDimensions()`.
    pub fn get_dimensions(&self) -> String {
        if self.is_package
            || self
                .definition
                .as_ref()
                .is_some_and(|definition| definition.has_behaviour(ItemBehaviour::WallItem))
        {
            return String::new();
        }

        match &self.definition {
            Some(definition) => format!(
                "{},{}",
                definition.get_length(),
                definition.get_width()
            ),
            None => String::new(),
        }
    }

    /// Mirrors `getSaleCode()`.
    pub fn get_sale_code(&self) -> &str {
        &self.sale_code
    }

    /// Mirrors `getPageS()`.
    pub fn get_page_s(&self) -> Vec<i32> {
        self.pages.clone()
    }

    /// Mirrors `getDefinition()`.
    pub fn get_definition(&self) -> Option<ItemDefinition> {
        self.definition.clone()
    }

    /// Mirrors `getPrice()`.
    pub fn get_price(&self) -> i32 {
        self.price
    }

    /// Mirrors `setPrice(int)`.
    pub fn set_price(&mut self, price: i32) {
        self.price = price;
    }

    /// Mirrors `getItemSpecialId()`.
    pub fn get_item_special_id(&self) -> i32 {
        self.item_special_id
    }

    /// Mirrors `isPackage()`.
    pub fn is_package(&self) -> bool {
        self.is_package
    }

    /// Mirrors `getPackages()`.
    pub fn get_packages(&self) -> Vec<CataloguePackage> {
        self.packages.clone()
    }

    /// Mirrors `addPackage(CataloguePackage)`.
    pub fn add_package(&mut self, package: CataloguePackage) {
        self.packages.push(package);
    }

    /// Mirrors `copy()`.
    pub fn copy(&self) -> Self {
        Self::with_pages(
            self.id,
            &self.sale_code,
            self.pages.clone(),
            self.order_id,
            self.price,
            self.is_hidden,
            self.definition.as_ref().map(|definition| definition.get_id()).unwrap_or(0),
            self.item_special_id,
            &self.name,
            &self.description,
            self.is_package,
            &self.package_name,
            &self.package_description,
        )
    }

    /// Mirrors `isHidden()`.
    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    /// Mirrors `hasPage(int)`.
    pub fn has_page(&self, page_id: i32) -> bool {
        self.pages.contains(&page_id)
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }
}

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod digital_collection {
    use ink::prelude::{string::String, vec::Vec};

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum CollectionError {
        ItemNotFound,
    }

    #[ink(storage)]
    pub struct DigitalCollection {
        user_collections: Vec<String>,
        collection_count: u32,
    }

    impl DigitalCollection {
        #[ink(constructor)]
        pub fn create_new() -> Self {
            Self {
                user_collections: Vec::new(),
                collection_count: 0,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::create_new()
        }

        #[ink(message)]
        pub fn add_to_collection(&mut self, item_name: String) {
            self.user_collections.push(item_name);
            self.collection_count += 1;
        }

        #[ink(message)]
        pub fn view_collection(&self) -> Vec<String> {
            self.user_collections.clone()
        }

        #[ink(message)]
        pub fn remove_from_collection(&mut self, index: u32) -> Result<(), CollectionError> {
            let idx = index as usize;
            
            if idx >= self.user_collections.len() {
                return Err(CollectionError::ItemNotFound);
            }
            
            self.user_collections.remove(idx);
            self.collection_count = self.collection_count.saturating_sub(1);
            
            Ok(())
        }
        
        #[ink(message)]
        pub fn collection_size(&self) -> u32 {
            self.collection_count
        }
        
        #[ink(message)]
        pub fn get_item(&self, index: u32) -> Option<String> {
            if (index as usize) < self.user_collections.len() {
                Some(self.user_collections[index as usize].clone())
            } else {
                None
            }
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[ink::test]
        fn basic_operations_work() {
            let mut collection = DigitalCollection::default();
            let test_item = String::from("digital item");
            
            collection.add_to_collection(test_item.clone());
            let items = collection.view_collection();
            assert_eq!(items.len(), 1);
            assert_eq!(items[0], test_item);
            
            assert_eq!(collection.remove_from_collection(0), Ok(()));
            assert_eq!(collection.view_collection().len(), 0);
        }
        
        #[ink::test]
        fn remove_invalid_index_fails() {
            let mut collection = DigitalCollection::default();
            collection.add_to_collection(String::from("test item"));
            assert_eq!(collection.remove_from_collection(1), Err(CollectionError::ItemNotFound));
        }
        
        #[ink::test]
        fn collection_size_works() {
            let mut collection = DigitalCollection::default();
            assert_eq!(collection.collection_size(), 0);
            
            collection.add_to_collection(String::from("item 1"));
            collection.add_to_collection(String::from("item 2"));
            assert_eq!(collection.collection_size(), 2);
            
            let _ = collection.remove_from_collection(0);
            assert_eq!(collection.collection_size(), 1);
        }
        
        #[ink::test]
        fn get_item_works() {
            let mut collection = DigitalCollection::default();
            let item = String::from("special item");
            
            collection.add_to_collection(item.clone());
            assert_eq!(collection.get_item(0), Some(item));
            assert_eq!(collection.get_item(1), None);
        }
    }
}
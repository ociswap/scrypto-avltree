use lazy_static::lazy_static;
use radix_engine::{
    blueprints::package::PackageDefinition,
    system::system_modules::execution_trace::ResourceSpecifier::Amount,
};
use scrypto::prelude::*;
use scrypto_testenv::*;
use std::mem;
use transaction::builder::ManifestBuilder;



lazy_static! {
    static ref PACKAGE: (Vec<u8>, PackageDefinition) = compile_package(this_package!());
}
pub struct TestHelper {
    env: TestEnvironment,
    tree_address: Option<ComponentAddress>,
}
impl TestHelperExecution for TestHelper {
    fn env(&mut self) -> &mut TestEnvironment {
        &mut self.env
    }
}
impl TestHelper {
    pub fn new() -> TestHelper {
        // let environment = TestEnvironment::new(&PACKAGE);
        let env = TestEnvironment::new(vec![("test", &PACKAGE)]);


        TestHelper {
            env: env,
            tree_address: None,
        }
    }

    pub fn instantiate(
        &mut self,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_function(
            self.env.package_address("test"),
            "AVLContainer",
            "instantiate",
            manifest_args!(),
        );
        // To support instruction labels we are tracking:
        // instruction_count = the total amount of new instructions added in this function
        // label_instruction_id = (local) instruction id which you want to assign to the label
        // after the ManifestBuilder supports labels upstream this can be simplified
        self.env.new_instruction("instantiate", 1, 0);
        self
    }
    pub fn instantiate_default(
        &mut self,
        verbose: bool,
    ) -> Receipt {
        self.instantiate();
        let receipt = self.execute_expect_success(verbose);
        let pool_address: ComponentAddress = receipt.outputs("instantiate")[0];
        self.tree_address = Some(pool_address);
        receipt
    }

    pub fn insert(
        &mut self,
        key: i32,
        value: i32,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "insert",
            manifest_args!(key, value),
        );
        self.env.new_instruction("insert", 1, 0);
        self
    }
    pub fn get(&mut self, key: i32) {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get",
            manifest_args!(key),
        );
        self.env.new_instruction("get", 1, 0);
    }
    pub fn delete(&mut self, key: i32) {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "delete",
            manifest_args!(key),
        );
        self.env.new_instruction("delete", 1, 0);
    }
    pub fn check_health(
        &mut self,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "check_health",
            manifest_args!(),
        );
        self.env.new_instruction("check_health", 1, 0);
        self
    }
    pub fn print(
        &mut self,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "print",
            manifest_args!(),
        );
        self.env.new_instruction("print", 1, 0);
        self
    }
    pub fn update_values(
        &mut self,
        start_key: i32,
        end_key: i32,
        value: i32,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "update_values",
            manifest_args!(start_key, end_key, value),
        );
        self.env.new_instruction("update_values", 1, 0);
        self
    }
    pub fn get_range_back_both_included(
        &mut self,
        key1: i32,
        key2: i32,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_back_both_included",
            manifest_args!(key1, key2),
        );
        self.env.new_instruction("get_range_back_both_included", 1, 0);
        self
    }
    pub fn get_range_back_both_excluded(
        &mut self,
        key1: i32,
        key2: i32,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_back_both_excluded",
            manifest_args!(key1, key2),
        );
        self.env.new_instruction("get_range_back_both_excluded", 1, 0);
        self
    }
    pub fn get_range_back(
        &mut self,
        key1: i32,
        key2: i32,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_back",
            manifest_args!(key1, key2),
        );
        self.env.new_instruction("get_range_back", 1, 0);
        self
    }
    pub fn get_range(
        &mut self,
        key1: i32,
        key2: i32,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range",
            manifest_args!(key1, key2),
        );
        self.env.new_instruction("get_range", 1, 0);
        self
    }
}

pub fn test_range(vector: Vec<i32>, to_delete: Vec<i32>) {
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);
    for i in vector.iter() {
        println!("inserting {:?}", i);
        helper.insert(*i, *i);
        // helper.print();
        helper.check_health();
        helper.execute_expect_success(true);
    }

    let minimum = i32::MIN;
    let maximum = i32::MAX;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("to_delete: {:?}", to_delete);
    println!("vector: {:?}", vector);
    println!("Output: {:?}", output);
    let mut last = i32::MIN;
    for i in output.clone() {
        assert!(last < i, "range_not_sorted {:?}", vector);
        last = i;
    }
    for i in vector.clone() {
        assert!(output.contains(&i), "i not contained in the tree {}", i);
    }
    for i in to_delete.iter().rev() {
        helper.delete(*i);
        // helper.print();
        // println!("Deleting {}", i);
        helper.check_health();
        helper.execute_expect_success(true);
    }
    helper.print();
    let mut minimum = i32::MAX;
    let mut maximum = i32::MIN;
    for i in vector.clone() {
        if i < minimum {
            minimum = i;
        }
        if i > maximum {
            maximum = i;
        }
    }
    // Maximum is exclusive.
    helper.get_range(minimum, maximum + 1);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("to_delete: {:?}", to_delete);
    println!("vector: {:?}", vector);
    println!("Output: {:?}", output);
    for i in vector.iter() {
        assert!(output.contains(&i) || to_delete.contains(&i), "i not contained in the tree {}", i);
    }
    for i in output.iter() {
        assert!(vector.contains(&i) && !to_delete.contains(&i), "elements in the tree that should be deleted or present {}", i);
    }
}

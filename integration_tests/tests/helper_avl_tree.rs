use lazy_static::lazy_static;
use radix_engine::blueprints::package::PackageDefinition;
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
        let env = TestEnvironment::new(vec![("test", &PACKAGE)]);

        TestHelper {
            env,
            tree_address: None,
        }
    }

    pub fn instantiate(&mut self) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_function(
            self.env.package_address("test"),
            "AvlTestWrapper",
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

    pub fn instantiate_default(&mut self, verbose: bool) -> Receipt {
        self.instantiate();
        let receipt = self.execute_expect_success(verbose);
        let pool_address: ComponentAddress = receipt.outputs("instantiate")[0];
        self.tree_address = Some(pool_address);
        receipt
    }

    pub fn insert(&mut self, key: i32, value: i32) -> &mut TestHelper {
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
        self.env.manifest_builder =
            manifest_builder.call_method(self.tree_address.unwrap(), "get", manifest_args!(key));
        self.env.new_instruction("get", 1, 0);
    }

    pub fn remove(&mut self, key: i32) {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder =
            manifest_builder.call_method(self.tree_address.unwrap(), "remove", manifest_args!(key));
        self.env.new_instruction("remove", 1, 0);
    }

    pub fn check_health(&mut self) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "check_health",
            manifest_args!(),
        );
        self.env.new_instruction("check_health", 1, 0);
        self
    }

    pub fn print(&mut self) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder =
            manifest_builder.call_method(self.tree_address.unwrap(), "print", manifest_args!());
        self.env.new_instruction("print", 1, 0);
        self
    }

    pub fn update_values_max_iters(
        &mut self,
        start_key: i32,
        end_key: i32,
        max_iters: i32,
        value: i32,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "update_values_max_iters",
            manifest_args!(start_key, end_key, max_iters, value),
        );
        self.env.new_instruction("update_values_max_iters", 1, 0);
        self
    }
    pub fn update_values(&mut self, start_key: i32, end_key: i32, value: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "update_values",
            manifest_args!(start_key, end_key, value),
        );
        self.env.new_instruction("update_values", 1, 0);
        self
    }

    pub fn get_range_back_both_included(&mut self, key1: i32, key2: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_back_both_included",
            manifest_args!(key1, key2),
        );
        self.env
            .new_instruction("get_range_back_both_included", 1, 0);
        self
    }

    pub fn get_range_back_both_excluded(&mut self, key1: i32, key2: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_back_both_excluded",
            manifest_args!(key1, key2),
        );
        self.env
            .new_instruction("get_range_back_both_excluded", 1, 0);
        self
    }

    pub fn get_range_back(&mut self, key1: i32, key2: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_back",
            manifest_args!(key1, key2),
        );
        self.env.new_instruction("get_range_back", 1, 0);
        self
    }

    pub fn get_range(&mut self, key1: i32, key2: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range",
            manifest_args!(key1, key2),
        );
        self.env.new_instruction("get_range", 1, 0);
        self
    }

    pub fn get_range_both_included(&mut self, key1: i32, key2: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_both_included",
            manifest_args!(key1, key2),
        );
        self.env.new_instruction("get_range_both_included", 1, 0);
        self
    }

    pub fn get_range_both_excluded(&mut self, key1: i32, key2: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_both_excluded",
            manifest_args!(key1, key2),
        );
        self.env.new_instruction("get_range_both_excluded", 1, 0);
        self
    }

    pub fn get_range_success(
        &mut self,
        key1: i32,
        key2: i32,
        output_expected: Vec<(i32, i32)>,
        verbose: bool,
    ) {
        let receipt = self.get_range(key1, key2).execute_expect_success(verbose);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range");
        assert_eq!(output, vec![output_expected]);
    }

    pub fn get_range_back_success(
        &mut self,
        key1: i32,
        key2: i32,
        output_expected: Vec<(i32, i32)>,
        verbose: bool,
    ) {
        let receipt = self
            .get_range_back(key1, key2)
            .execute_expect_success(verbose);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back");
        assert_eq!(output, vec![output_expected]);
    }
}

pub fn to_key_values(vector: &Vec<i32>) -> Vec<(i32, i32)> {
    vector
        .iter()
        .zip(vector.iter())
        .map(|(a, b)| (*a, *b))
        .collect()
}

pub fn test_range(mut vector: Vec<i32>, to_delete: Vec<i32>) {
    println!("to_delete: {:?}", to_delete);
    println!("vector: {:?}", vector);
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);
    for i in vector.iter() {
        println!("inserting {:?}", i);
        helper.insert(*i, *i);
        helper.check_health();
        helper.execute_expect_success(true);
    }

    vector.sort();
    let mut key_values: Vec<(i32, i32)> = to_key_values(&vector);

    helper.get_range_success(i32::MIN, i32::MAX, key_values.clone(), true);

    for i in to_delete.iter().rev() {
        helper.remove(*i);
        helper.check_health();
        // helper.print();
        helper.execute_expect_success(true);
    }

    key_values.retain(|(k, _)| !to_delete.contains(&k));

    helper.get_range_success(i32::MIN, i32::MAX, key_values, true);
}

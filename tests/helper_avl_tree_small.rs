use std::mem;

use lazy_static::lazy_static;
use radix_engine::blueprints::package::PackageDefinition;
use scrypto::prelude::*;
use scrypto_testenv::*;
use transaction::builder::ManifestBuilder;

impl TestHelperExecution for TestHelper {
    fn env(&mut self) -> &mut TestEnvironment {
        &mut self.env
    }
}
lazy_static! {
    static ref PACKAGE: (Vec<u8>, PackageDefinition) = compile_package(this_package!());
}

pub struct TestHelper {
    env: TestEnvironment,
    tree_address: Option<ComponentAddress>,
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
            "AvlTestWrapperU16",
            "instantiate",
            manifest_args!()
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

    pub fn insert(&mut self, key: u16, value: ()) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "insert",
            manifest_args!(key, value)
        );
        self.env.new_instruction("insert", 1, 0);
        self
    }

    pub fn delete(&mut self, key: u16) {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "delete",
            manifest_args!(key)
        );
        self.env.new_instruction("delete", 1, 0);
    }

    pub fn check_health(&mut self) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "check_health",
            manifest_args!()
        );
        self.env.new_instruction("check_health", 1, 0);
        self
    }

    pub fn get_range(&mut self, key1: u16, key2: u16) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range",
            manifest_args!(key1, key2)
        );
        self.env.new_instruction("get_range", 1, 0);
        self
    }

    pub fn get_range_success(
        &mut self,
        key1: u16,
        key2: u16,
        output_expected: Vec<((), ())>,
        verbose: bool
    ) {
        let receipt = self.get_range(key1, key2).execute_expect_success(verbose);
        let output: Vec<Vec<((), ())>> = receipt.outputs("get_range");
        assert_eq!(output, vec![output_expected]);
    }
}

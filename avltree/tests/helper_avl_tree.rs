use lazy_static::lazy_static;
use radix_engine::{
    blueprints::package::PackageDefinition,
    system::system_modules::execution_trace::ResourceSpecifier::Amount,
};
use scrypto::prelude::*;
use scrypto_testenv::*;
use std::mem;
use transaction::builder::ManifestBuilder;

impl TestHelperExecution for TestHelper {
    fn environment(&mut self) -> &mut TestEnvironment {
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
        let environment = TestEnvironment::new(&PACKAGE);


        TestHelper {
            env: environment,
            tree_address: None,
        }
    }

    pub fn instantiate(
        &mut self,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_function(
                    self.env.package_address,
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
        value: i32
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        manifest_builder.call_method(
            self.tree_address.unwrap(),
            "insert",
            manifest_args!(key, value)
        );
        self.env.new_instruction("insert", 1, 0);
        self
    }
    pub fn delete(&mut self, key: i32) {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        manifest_builder.call_method(
            self.tree_address.unwrap(),
            "delete",
            manifest_args!(key)
        );
        self.env.new_instruction("delete", 1, 0);
    }
    pub fn check_health(
        &mut self,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        manifest_builder.call_method(
            self.tree_address.unwrap(),
            "check_health",
            manifest_args!()
        );
        self.env.new_instruction("check_health", 1, 0);
        self
    }
    pub fn print(
        &mut self,
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        manifest_builder.call_method(
            self.tree_address.unwrap(),
            "print",
            manifest_args!()
        );
        self.env.new_instruction("print", 1, 0);
        self
    }
    pub fn get_range(
        &mut self,
        key1: i32,
        key2: i32
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range",
            manifest_args!(key1, key2)
        );
        self.env.new_instruction("get_range", 1, 0);
        self
    }
    pub fn get_range_mut(
        &mut self,
        key1: i32,
        key2: i32
    ) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_mut",
            manifest_args!(key1, key2)
        );
        self.env.new_instruction("get_range_mut", 1, 0);
        self
    }
    //
    // fn execute(&mut self, verbose: bool) -> Receipt {
    //     self.env.manifest_builder.call_method(
    //         self.account_component,
    //         "deposit_batch",
    //         manifest_args!(ManifestExpression::EntireWorktop)
    //     );
    //     let receipt = self.test_runner.execute_manifest_ignoring_fee(
    //         self.manifest_builder.build(),
    //         vec![NonFungibleGlobalId::from_public_key(&self.public_key)]
    //     );
    //     let instruction_mapping = self.instruction_ids_by_label.clone();
    //     self.reset_manifest();
    //     if verbose {
    //         println!("{:?}\n", receipt);
    //     }
    //     Receipt { receipt, instruction_ids_by_label: instruction_mapping }
    // }
    //
    // pub fn reset_manifest(&mut self) {
    //     self.manifest_builder = ManifestBuilder::new();
    //     self.instruction_ids_by_label = HashMap::new();
    //     self.instruction_counter = INSTRUCTION_COUNTER;
    // }
    //
    // pub fn execute_success(&mut self, verbose: bool) -> Receipt {
    //     let receipt = self.execute(verbose);
    //     receipt.receipt.expect_commit_success();
    //     receipt
    // }
    // fn new_instruction(
    //     &mut self,
    //     label: &str,
    //     instruction_count: usize,
    //     local_instruction_id: usize
    // ) {
    //     self.instruction_ids_by_label
    //         .entry(label.to_string())
    //         .or_default()
    //         .push(self.instruction_counter + local_instruction_id);
    //     self.instruction_counter += instruction_count;
    // }
}

pub fn test_range(vector: Vec<i32>, to_delete: Vec<i32>) {
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);
    for i in vector.iter() {
        helper.insert(*i, *i);
        // helper.print();
        helper.check_health();
        helper.execute_expect_success(true);
    }

    let mut minimum = i32::MIN;
    let mut maximum = i32::MAX;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("to_delete: {:?}", to_delete);
    println!("vector: {:?}", vector);
    println!("Output: {:?}", output);
    let mut last= i32::MIN;
    for i in output.clone(){
        assert!(last < i, "range_not_sorted {:?}", vector);
        last = i;
    }
    for i in vector.clone() {
        assert!(output.contains(&i), "i not contained in the tree {}", i);
    }
    for i in to_delete.iter().rev() {
        helper.delete(*i);
        // helper.print();
        println!("Deleting {}", i);
        helper.check_health();
        helper.execute_expect_success(true);
    }
    // helper.print();
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
    helper.get_range(minimum, maximum);
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

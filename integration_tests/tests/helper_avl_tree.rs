use lazy_static::lazy_static;
use radix_engine::blueprints::package::PackageDefinition;
use scrypto::prelude::*;
use scrypto_testenv::*;
use std::mem;
use std::time::SystemTime;
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
    pub fn execute_success(&mut self, verbose: bool) -> Result<Receipt, String> {
        let receipt = self.execute(verbose);
        if receipt.execution_receipt.is_commit_success() {
            Ok(receipt)
        } else {
            println!("{:?}", receipt.execution_receipt);
            println!("=========================");
            Err(format!("{:?}", receipt.execution_receipt))
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
    pub fn batch_insert(&mut self, keys: Vec<i32>, values: Vec<i32>) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "batch_insert",
            manifest_args!(keys, values),
        );
        self.env.new_instruction("batch_insert", 1, 0);
        self
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

    pub fn get(&mut self, key: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder =
            manifest_builder.call_method(self.tree_address.unwrap(), "get", manifest_args!(key));
        self.env.new_instruction("get", 1, 0);
        self
    }

    pub fn remove(&mut self, key: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder =
            manifest_builder.call_method(self.tree_address.unwrap(), "remove", manifest_args!(key));
        self.env.new_instruction("remove", 1, 0);
        self
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
    pub fn update_value(&mut self, key: i32, value: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "update_value",
            manifest_args!(key, value),
        );
        self.env.new_instruction("update_value", 1, 0);
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

    pub fn get_range_mut_both_included(&mut self, key1: i32, key2: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_mut_both_included",
            manifest_args!(key1, key2),
        );
        self.env
            .new_instruction("get_range_mut_both_included", 1, 0);
        self
    }

    pub fn get_range_mut_both_excluded(&mut self, key1: i32, key2: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_mut_both_excluded",
            manifest_args!(key1, key2),
        );
        self.env
            .new_instruction("get_range_mut_both_excluded", 1, 0);
        self
    }

    pub fn get_range_back_mut_both_included(&mut self, key1: i32, key2: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_back_mut_both_included",
            manifest_args!(key1, key2),
        );
        self.env
            .new_instruction("get_range_back_mut_both_included", 1, 0);
        self
    }

    pub fn get_range_back_mut_both_excluded(&mut self, key1: i32, key2: i32) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_back_mut_both_excluded",
            manifest_args!(key1, key2),
        );
        self.env
            .new_instruction("get_range_back_mut_both_excluded", 1, 0);
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
    pub fn get_range_safe(
        &mut self,
        key1: i32,
        key2: i32,
        output_expected: &Vec<(i32, i32)>,
        verbose: bool,
    ) -> Result<Vec<(i32, i32)>, String> {
        let receipt = self.get_range(key1, key2).execute_success(verbose)?;
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range");

        if output.len() == 0 {
            return Err(format!(
                "output is empty with key1: {} and key2: {}",
                key1, key2
            ));
        }
        let output = output.get(0).unwrap();
        if output != output_expected {
            return Err(format!(
                "output is not as expected with key1: {} and key2: {}\n expected: {:?}\n got: {:?}",
                key1, key2, output_expected, output
            ));
        }
        Ok(output.clone())
    }

    pub fn get_range_success(
        &mut self,
        key1: i32,
        key2: i32,
        output_expected: &Vec<(i32, i32)>,
        verbose: bool,
    ) {
        if let Err(e) = self.get_range_safe(key1, key2, output_expected, verbose) {
            panic!("{:?}", e);
        }
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
#[derive(Debug)]
pub enum Function {
    Insert(i32),
    Delete(i32),
}
pub fn test_with_functions(
    initial_vector: &Vec<i32>,
    functions: &Vec<Function>,
) -> Result<(), String> {
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);
    let batch_size = 50;
    let batched_vector: Vec<Vec<i32>> = initial_vector
        .chunks(batch_size)
        .map(|x| x.to_vec())
        .collect();
    for i in batched_vector.iter() {
        println!("inserting {:?}", i);
        helper
            .batch_insert(i.clone(), i.clone())
            .execute_success(true)?;
        helper.check_health().execute_success(true)?;
    }

    let mut initial_vector = initial_vector.clone();
    initial_vector.sort();
    let mut key_values: Vec<(i32, i32)> = to_key_values(&initial_vector);

    helper.get_range_safe(i32::MIN, i32::MAX, &key_values, false)?;

    for function in functions.iter() {
        println!("function: {:?}", function);
        match function {
            Function::Insert(i) => {
                helper.insert(*i, *i);
                helper.check_health();
                key_values.push((*i, *i));
                key_values.sort();
            }
            Function::Delete(i) => {
                helper.remove(*i);
                helper.check_health();
                key_values.retain(|(k, _)| k != i);
            }
        }
        let receipt = helper.execute_success(false)?;
        if receipt.execution_receipt.is_commit_failure() {
            return Err(format!("Error: {:?}", receipt.execution_receipt));
        };
    }
    helper.check_health();
    helper.check_health().execute_success(false)?;
    helper.get_range_safe(i32::MIN, i32::MAX, &key_values, true)?;
    helper.check_health().execute_success(false)?;
    for i in initial_vector.iter() {
        helper.remove(*i).execute_success(false)?;
        helper.check_health().execute_success(false)?;
    }
    Ok(())
}
pub fn test_range(mut vector: Vec<i32>, to_delete: Vec<i32>) {
    _test_range(vector, to_delete, true);
}

pub fn write_costs_csv_test_range(mut vector: Vec<i32>, to_delete: Vec<i32>) {
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);
    let base_receipt = helper.get(i32::MIN).execute_expect_success(true);
    let base_cost = base_receipt.execution_receipt.fee_summary.total_cost();

    let csv_path = "../../../plot_costs/batched_costs.csv";
    let mut wtr = csv::Writer::from_path(csv_path).unwrap();
    let batch_size = 25;
    let start = SystemTime::now();
    let batched_vector: Vec<Vec<i32>> = vector.chunks(batch_size).map(|x| x.to_vec()).collect();
    for (idx, i) in batched_vector.iter().enumerate() {
        let start = SystemTime::now();
        helper.batch_insert(i.clone(), i.clone());
        let receipt: Receipt = helper.execute_expect_success(true);
        let cost = receipt.execution_receipt.fee_summary.total_cost();
        let cost = (cost - base_cost) / batch_size;
        let end = SystemTime::now();
        let time = end.duration_since(start).unwrap().as_millis();
        let normalized_time = time / batch_size as u128;
        println!("time: {:?}", normalized_time);
        println!("inserting {}:{:?}, ", idx * batch_size, i);
        println!("cost: {:?}", cost);
        wtr.write_record(&[(idx * batch_size).to_string(), cost.to_string()])
            .unwrap();
        wtr.flush();
    }

    helper.check_health();
    helper.execute_expect_success(true);

    vector.sort();
    let mut key_values: Vec<(i32, i32)> = to_key_values(&vector);

    helper.get_range_success(i32::MIN, i32::MAX, &key_values, true);

    let csv_path = "../../../plot_costs/delete_batched_costs.csv";
    let mut wtr = csv::Writer::from_path(csv_path).unwrap();
    for (idx, i) in to_delete.iter().rev().enumerate() {
        println!("deleting {},{:?}", idx, i);
        helper.remove(*i);
        let receipt = helper.execute_expect_success(true);
        let cost = receipt.execution_receipt.fee_summary.total_cost();
        let cost = (cost - base_cost) / batch_size;
        println!("cost: {:?}", cost);
        wtr.write_record(&[(idx * batch_size).to_string(), cost.to_string()])
            .unwrap();
        if idx % 10 == 0 {
            wtr.flush();
        }
    }
    helper.check_health();
    helper.execute_expect_success(true);

    key_values.retain(|(k, _)| !to_delete.contains(&k));
    helper.get_range_success(i32::MIN, i32::MAX, &key_values, true);
}
pub fn _test_range(mut vector: Vec<i32>, to_delete: Vec<i32>, expensive: bool) {
    if expensive {
        println!("vector: {:?}", vector);
        println!("to_delete: {:?}", to_delete);
    }
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);
    for (idx, i) in vector.iter().enumerate() {
        println!("inserting {}:{:?}, ", idx, i);
        helper.insert(*i, *i);
        if expensive {
            helper.check_health();
            helper.execute_expect_success(true);
        } else {
            helper.execute_expect_success(true);
        }
    }
    if !expensive {
        helper.check_health();
        helper.execute_expect_success(true);
    }

    vector.sort();
    let mut key_values: Vec<(i32, i32)> = to_key_values(&vector);

    helper.get_range_success(i32::MIN, i32::MAX, &key_values, true);

    for (idx, i) in to_delete.iter().rev().enumerate() {
        println!("deleting {},{:?}", idx, i);
        helper.remove(*i);
        if expensive {
            helper.check_health();
        }
        // helper.print();
        helper.execute_expect_success(true);
    }
    if !expensive {
        helper.check_health();
        helper.execute_expect_success(true);
    }

    key_values.retain(|(k, _)| !to_delete.contains(&k));

    helper.get_range_success(i32::MIN, i32::MAX, &key_values, true);
}

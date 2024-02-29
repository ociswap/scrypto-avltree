use lazy_static::lazy_static;
use radix_engine::blueprints::package::PackageDefinition;
use scrypto::prelude::*;
use scrypto_testenv::*;
use std::fs;
use std::mem;
use std::time::SystemTime;
use transaction::builder::ManifestBuilder;

// #[derive(Clone, Debug, ScryptoSbor, PartialEq, Eq)]
// struct Example {
// }

#[derive(Clone, Debug, ScryptoSbor, PartialEq, Eq)]
pub struct Tick {
    pub delta_liquidity: PreciseDecimal,
    pub total_liquidity: PreciseDecimal,
    pub price_sqrt: PreciseDecimal,
    pub x_fee_outside: PreciseDecimal,
    pub y_fee_outside: PreciseDecimal,
}

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
            "AvlTestWrapperTick",
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
    // pub fn batch_insert(&mut self, keys: Vec<i32>, values: Vec<Tick>) -> &mut TestHelper {
    //     let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
    //     self.env.manifest_builder = manifest_builder.call_method(
    //         self.tree_address.unwrap(),
    //         "batch_insert",
    //         manifest_args!(keys, values),
    //     );
    //     self.env.new_instruction("batch_insert", 1, 0);
    //     self
    // }

    pub fn insert(&mut self, key: i32, value: Tick) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        let transfer_value = (
            value.delta_liquidity,
            value.total_liquidity,
            value.price_sqrt,
            value.x_fee_outside,
            value.y_fee_outside,
        );
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "insert",
            manifest_args!(key, transfer_value),
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
    pub fn update_value(&mut self, key: i32, value: Tick) -> &mut TestHelper {
        let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
        let transfer_value = (
            value.delta_liquidity,
            value.total_liquidity,
            value.price_sqrt,
            value.x_fee_outside,
            value.y_fee_outside,
        );
        self.env.manifest_builder = manifest_builder.call_method(
            self.tree_address.unwrap(),
            "update_value",
            manifest_args!(key, transfer_value),
        );
        self.env.new_instruction("update_value", 1, 0);
        self
    }

    // pub fn update_values(&mut self, start_key: i32, end_key: i32, value: Tick) -> &mut TestHelper {
    //     let manifest_builder = mem::replace(&mut self.env.manifest_builder, ManifestBuilder::new());
    //     self.env.manifest_builder = manifest_builder.call_method(
    //         self.tree_address.unwrap(),
    //         "update_values",
    //         manifest_args!(start_key, end_key, value),
    //     );
    //     self.env.new_instruction("update_values", 1, 0);
    //     self
    // }

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
        output_expected: &Vec<(i32, Tick)>,
        verbose: bool,
    ) -> Result<Vec<(i32, Tick)>, String> {
        let receipt = self.get_range(key1, key2).execute_success(verbose)?;
        let output: Vec<Vec<(i32, Tick)>> = receipt.outputs("get_range");

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
        output_expected: &Vec<(i32, Tick)>,
        verbose: bool,
    ) -> Vec<(i32, Tick)> {
        let result = self.get_range_safe(key1, key2, output_expected, verbose);
        if let Err(e) = result {
            panic!("{:?}", e);
        }
        result.unwrap()
    }

    pub fn get_range_back_success(
        &mut self,
        key1: i32,
        key2: i32,
        output_expected: Vec<(i32, Tick)>,
        verbose: bool,
    ) {
        let receipt = self
            .get_range_back(key1, key2)
            .execute_expect_success(verbose);
        let output: Vec<Vec<(i32, Tick)>> = receipt.outputs("get_range_back");
        assert_eq!(output, vec![output_expected]);
    }
}

pub fn write_costs_csv_test_range(vector: Vec<i32>) {
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);
    let base_receipt = helper.get(i32::MIN).execute_expect_success(true);
    let base_cost = base_receipt.execution_receipt.fee_summary.total_cost();

    // let csv_path = "../../../projects/plot_costs/batched_costs.csv";
    let csv_path = "plot_costs/insert_delete_costs.csv";
    fs::create_dir("plot_costs").unwrap_or_default();
    let mut wtr = csv::Writer::from_path(csv_path).unwrap();
    let tick: Tick = Tick {
        delta_liquidity: PreciseDecimal::ZERO,
        total_liquidity: PreciseDecimal::ZERO,
        price_sqrt: PreciseDecimal::ZERO,
        x_fee_outside: PreciseDecimal::ZERO,
        y_fee_outside: PreciseDecimal::ZERO,
    };
    let shift = 10;
    for i in 0..shift {
        helper.insert(vector[i], tick.clone());
        helper.execute_expect_success(true);
    }
    let batch_size = 3;
    let zipped = vector.iter().zip(vector.iter().cycle().skip(shift));
    // let zipped = zipped.collect::<Vec<(&i32,&i32)>>();
    // println!("zipped: {:?}", zipped);
    // panic!();

    for (idx, (&delete, &insert)) in zipped.enumerate() {
        let start = SystemTime::now();
        helper.remove(delete);
        helper.insert(insert, tick.clone());
        helper.insert(delete, tick.clone());
        let receipt: Receipt = helper.execute_expect_success(true);
        let cost = receipt.execution_receipt.fee_summary.total_cost();
        let cost = (cost - base_cost) / batch_size;
        let end = SystemTime::now();
        let time = end.duration_since(start).unwrap().as_millis();
        let normalized_time = time / batch_size as u128;
        // println!("time: {:?}", normalized_time);
        // println!("inserting {}:{:?}, ", idx * batch_size, i);
        // println!("cost: {:?}", cost);
        wtr.write_record(&[(shift + idx * batch_size).to_string(), cost.to_string()])
            .unwrap();
        wtr.flush();
    }
}

use pretty_assertions::{ assert_eq, assert_ne };
use radix_engine::{
    system::kernel_modules::execution_trace::{
        ResourceSpecifier,
        ResourceSpecifier::Amount,
        ResourceSpecifier::Ids,
        WorktopChange,
    },
    transaction::TransactionReceipt,
};
use scrypto::prelude::*;
use scrypto_unit::TestRunner;
use trangsaction::builder::ManifestBuilder;

#[macro_export]
macro_rules! nft_ids {
    ($($x:expr),*) => {
        {
            let mut temp_set = BTreeSet::new();
            $(
                temp_set.insert(NonFungibleLocalId::Integer($x.into()));
            )*
            temp_set
        }
    };
}

pub const MAX_TICK: i32 = 887272;
pub const MIN_TICK: i32 = -MAX_TICK;
pub const MAX_SUPPLY: Decimal = Decimal(
    BnumI256::from_digits([12919594847110692864, 54210108624275221, 0, 0])
); // 1000000000000000000

const INSTRUCTION_COUNTER: usize = 1; // execute_manifest_ignoring_fee adds one instruction automatically

pub struct TestHelper {
    pub test_runner: TestRunner,
    pub manifest_builder: ManifestBuilder,

    pub package_address: PackageAddress,
    pub public_key: EcdsaSecp256k1PublicKey,
    pub account_component: ComponentAddress,

    pub tree_address: Option<ComponentAddress>,

    instruction_counter: usize,
    instruction_ids_by_label: HashMap<String, Vec<usize>>,
}

impl TestHelper {
    pub fn new() -> TestHelper {
        let mut test_runner = TestRunner::builder().without_trace().build();

        let (public_key, _private_key, account_component) = test_runner.new_allocated_account();
        let package_address = test_runner.compile_and_publish(this_package!());

        let manifest_builder = ManifestBuilder::new();

        TestHelper {
            test_runner,
            manifest_builder,

            package_address,
            public_key,
            account_component,

            tree_address: None,

            instruction_counter: INSTRUCTION_COUNTER,
            instruction_ids_by_label: HashMap::new(),
        }
    }

    pub fn instantiate(
        &mut self,
    ) -> &mut TestHelper {
        self.manifest_builder.call_function(
            self.package_address,
            "AVLContainer",
            "instantiate",
            manifest_args!()
        );
        self.new_instruction("instantiate", 1, 0);
        self
    }

    pub fn instantiate_default(&mut self, verbose: bool) -> &mut TestHelper {
        self.instantiate();
        let receipt = self.execute_success(verbose);
        let tree_address: ComponentAddress = receipt.receipt
            .expect_commit_success()
            .output(1);
        self.tree_address = Some(tree_address);
        self
    }

    pub fn insert(
        &mut self,
        key: i32,
        value: i32
    ) -> &mut TestHelper {
        self.manifest_builder.call_method(
            self.tree_address.unwrap(),
            "insert",
            manifest_args!(key, value)
        );
        self.new_instruction("insert", 1, 0);
        self
    }
    pub fn delete(&mut self, key: i32) {
        self.manifest_builder.call_method(
            self.tree_address.unwrap(),
            "delete",
            manifest_args!(key)
        );
        self.new_instruction("delete", 1, 0);
    }
    pub fn check_health(
        &mut self,
    ) -> &mut TestHelper {
        self.manifest_builder.call_method(
            self.tree_address.unwrap(),
            "check_health",
            manifest_args!()
        );
        self.new_instruction("check_health", 1, 0);
        self
    }
    pub fn print(
        &mut self,
    ) -> &mut TestHelper {
        self.manifest_builder.call_method(
            self.tree_address.unwrap(),
            "print",
            manifest_args!()
        );
        self.new_instruction("print", 1, 0);
        self
    }
    pub fn get_range(
        &mut self,
        key1: i32,
        key2: i32
    ) -> &mut TestHelper {
        self.manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range",
            manifest_args!(key1, key2)
        );
        self.new_instruction("get_range", 1, 0);
        self
    }
    pub fn get_range_mut(
        &mut self,
        key1: i32,
        key2: i32
    ) -> &mut TestHelper {
        self.manifest_builder.call_method(
            self.tree_address.unwrap(),
            "get_range_mut",
            manifest_args!(key1, key2)
        );
        self.new_instruction("get_range_mut", 1, 0);
        self
    }

    fn execute(&mut self, verbose: bool) -> Receipt {
        self.manifest_builder.call_method(
            self.account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop)
        );
        let receipt = self.test_runner.execute_manifest_ignoring_fee(
            self.manifest_builder.build(),
            vec![NonFungibleGlobalId::from_public_key(&self.public_key)]
        );
        let instruction_mapping = self.instruction_ids_by_label.clone();
        self.reset_manifest();
        if verbose {
            println!("{:?}\n", receipt);
        }
        Receipt { receipt, instruction_ids_by_label: instruction_mapping }
    }

    pub fn reset_manifest(&mut self) {
        self.manifest_builder = ManifestBuilder::new();
        self.instruction_ids_by_label = HashMap::new();
        self.instruction_counter = INSTRUCTION_COUNTER;
    }

    pub fn execute_success(&mut self, verbose: bool) -> Receipt {
        let receipt = self.execute(verbose);
        receipt.receipt.expect_commit_success();
        receipt
    }
    fn new_instruction(
        &mut self,
        label: &str,
        instruction_count: usize,
        local_instruction_id: usize
    ) {
        self.instruction_ids_by_label
            .entry(label.to_string())
            .or_default()
            .push(self.instruction_counter + local_instruction_id);
        self.instruction_counter += instruction_count;
    }
}


pub struct Receipt {
    pub receipt: TransactionReceipt,
    pub instruction_ids_by_label: HashMap<String, Vec<usize>>,
}

impl Receipt {
    pub fn output_buckets(&self, instruction_label: &str) -> Vec<Vec<ResourceSpecifier>> {
        self.receipt.output_buckets(self.instruction_ids(instruction_label))
    }

    pub fn outputs<T>(&self, instruction_label: &str) -> Vec<T> where T: ScryptoDecode {
        self.receipt.outputs(self.instruction_ids(instruction_label))
    }

    fn instruction_ids(&self, instruction_label: &str) -> Vec<usize> {
        self.instruction_ids_by_label.get(&instruction_label.to_string()).unwrap().clone()
    }
}

pub trait TransactionReceiptOutputBuckets {
    fn output_buckets(&self, instruction_ids: Vec<usize>) -> Vec<Vec<ResourceSpecifier>>;
    fn outputs<T>(&self, instruction_ids: Vec<usize>) -> Vec<T> where T: ScryptoDecode;
}

impl TransactionReceiptOutputBuckets for TransactionReceipt {
    fn output_buckets(&self, instruction_ids: Vec<usize>) -> Vec<Vec<ResourceSpecifier>> {
        let worktop_changes = self.execution_trace.worktop_changes();
        instruction_ids
            .iter()
            .filter_map(|id| {
                let instruction_worktop_changes = worktop_changes.get(id).unwrap();
                Some(
                    instruction_worktop_changes
                        .iter()
                        .filter_map(|change| {
                            match change {
                                WorktopChange::Put(resource_specifier) =>
                                    Some(resource_specifier.clone()),
                                _ => None,
                            }
                        })
                        .collect()
                )
            })
            .collect()
    }

    fn outputs<T>(&self, instruction_ids: Vec<usize>) -> Vec<T> where T: ScryptoDecode {
        instruction_ids
            .iter()
            .filter_map(|id| { Some(self.expect_commit_success().output(*id)) })
            .collect()
    }
}

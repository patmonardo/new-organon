//! Values system walkthrough.
//!
//! A runnable tour of the runtime value layer:
//! - `ValueType` helpers
//! - `DefaultValue` fallbacks
#![allow(clippy::all)]
//! - `PrimitiveValues` factory (runtime parsing + smart conversions)
//!
//! Run with:
//!   cargo run -p gds --example values_system_walkthrough

mod enabled {
    use gds::types::{DefaultValue, ValueType};
    use gds::values::{
        DefaultFloatingPointValue, DefaultLongArray, DefaultLongValue, FromGdsValue, GdsValue,
        PrimitiveValues,
    };
    use gds::{Array, FloatingPointValue, IntegralArray, IntegralValue};
    use serde_json::json;

    pub fn main() {
        if let Err(err) = run() {
            eprintln!("values_system_walkthrough failed: {err}");
            std::process::exit(1);
        }
    }

    fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("=== Values system walkthrough ===\n");

        print_value_type_helpers();
        println!();

        demonstrate_default_values()?;
        println!();

        demonstrate_primitive_values_factory()?;
        println!();

        demonstrate_smart_converter()?;

        Ok(())
    }

    fn print_value_type_helpers() {
        let types = [
            ValueType::Long,
            ValueType::Double,
            ValueType::Boolean,
            ValueType::String,
            ValueType::LongArray,
        ];

        println!("Selected ValueTypes:");
        for vt in types {
            println!(
                "- {:<11} csv={} cypher={} compatible_with_string_array={}",
                vt.name(),
                vt.csv_name().unwrap_or("n/a"),
                vt.cypher_name(),
                vt.is_compatible_with(ValueType::StringArray)
            );
        }
    }

    fn demonstrate_default_values() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("DefaultValue conversions:");

        let long_default = DefaultValue::long(42);
        let double_default = DefaultValue::double(3.14);
        let bool_default = DefaultValue::boolean(true);
        let null_default = DefaultValue::null();

        println!(
            "- long -> {} (user_defined={})",
            long_default.long_value()?,
            long_default.is_user_defined()
        );
        println!(
            "- double -> {:.2} (user_defined={})",
            double_default.double_value()?,
            double_default.is_user_defined()
        );
        println!(
            "- boolean -> {} (user_defined={})",
            bool_default.boolean_value()?,
            bool_default.is_user_defined()
        );
        println!(
            "- null -> is_null_value={} (user_defined={})",
            null_default.is_null_value(),
            null_default.is_user_defined()
        );

        Ok(())
    }

    fn demonstrate_primitive_values_factory() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    {
        println!("PrimitiveValues factory:");

        let parsed_int = PrimitiveValues::of(&json!("123")).expect("string → int");
        let parsed_float = PrimitiveValues::of(&json!(2.718)).expect("float");
        let parsed_array = PrimitiveValues::of(&json!([1, 2, 3])).expect("array");

        let int_val = parsed_int
            .as_any()
            .downcast_ref::<DefaultLongValue>()
            .expect("long");
        println!(
            "- \"123\" parsed as ValueType::{:?}, long_value={}",
            int_val.value_type(),
            int_val.long_value()
        );

        let float_val = parsed_float
            .as_any()
            .downcast_ref::<DefaultFloatingPointValue>()
            .expect("float");
        println!(
            "- 2.718 parsed as ValueType::{:?}, double_value={}",
            float_val.value_type(),
            float_val.double_value()
        );

        let arr_val = parsed_array
            .as_any()
            .downcast_ref::<DefaultLongArray>()
            .expect("long array");
        println!(
            "- [1,2,3] parsed as ValueType::{:?}, len={}, elements={:?}",
            arr_val.value_type(),
            arr_val.length(),
            arr_val.long_array_value()
        );

        Ok(())
    }

    fn demonstrate_smart_converter() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Smart converter (FromGdsValue):");

        let float_val = PrimitiveValues::of(&json!(5.0)).expect("float");
        let as_i64 = i64::from_gds_value(float_val.as_ref())?;
        println!(
            "- float 5.0 converted to i64 via FromGdsValue -> {}",
            as_i64
        );

        // Boolean parsing via direct factory (json! parsing for bool isn't wired yet).
        let bool_val = PrimitiveValues::boolean_value(true);
        let as_i64 = i64::from_gds_value(bool_val.as_ref())?;
        println!(
            "- bool true converted to i64 via FromGdsValue -> {}",
            as_i64
        );

        Ok(())
    }
}

fn main() {
    enabled::main();
}

//! This module provides a helper to generate the MavLink XML definition from metadata.

use std::collections::BTreeSet;
use std::fmt::Write;

use crate::description::{DescribeMessages, Enum, Message, MessageField};
use crate::typing::MavType;

/// Generates a MAVLink XML dialect file from a type implementing both
/// `DescribeMessages` and `mavio::Dialect`.
///
/// The generated XML includes:
/// - XML declaration
/// - Dialect version and ID (if available)
/// - All unique enums referenced by message fields
/// - All messages with their fields
///
/// # Example output
/// ```xml
/// <?xml version="1.0"?>
/// <mavlink>
///     <version>1</version>
///     <enums>
///         <!-- enum definitions -->
///     </enums>
///     <messages>
///         <!-- message definitions -->
///     </messages>
/// </mavlink>
/// ```
pub fn make_dialect_file<D: DescribeMessages + mavio::Dialect>() -> String {
    let mut output = String::new();

    // XML declaration
    output.push_str("<?xml version=\"1.0\"?>\n\n");

    // Root element
    output.push_str("<mavlink>\n");

    // Version
    if let Some(version) = D::version() {
        let _ = writeln!(output, "    <version>{version}</version>");
    }

    // Dialect ID
    if let Some(dialect_id) = D::dialect() {
        let _ = writeln!(output, "    <dialect>{dialect_id}</dialect>");
    }

    // Collect all unique enums from message fields
    let enums = collect_unique_enums(D::MESSAGES);

    // Enums section
    if !enums.is_empty() {
        output.push_str("\n    <enums>\n");
        for enum_ in enums {
            write_enum(&mut output, enum_);
        }
        output.push_str("    </enums>\n");
    }

    // Messages section
    output.push_str("\n    <messages>\n");
    for message in D::MESSAGES {
        write_message(&mut output, message);
    }
    output.push_str("    </messages>\n");

    // Close root element
    output.push_str("</mavlink>\n");

    output
}

/// Collects all unique enums referenced by message fields.
fn collect_unique_enums(messages: &[Message]) -> Vec<&'static Enum> {
    let mut enum_set = BTreeSet::new();

    for message in messages {
        for field in message.fields {
            if let Some(enum_) = field.enum_ {
                // Use the enum's name as the key for uniqueness
                enum_set.insert(enum_.name);
            }
        }
    }

    // Collect the actual enum references
    let mut enums = Vec::new();
    for message in messages {
        for field in message.fields {
            if let Some(enum_) = field.enum_
                && enum_set.contains(enum_.name)
            {
                enums.push(enum_);
                enum_set.remove(enum_.name);
            }
        }
    }

    enums
}

/// Writes an enum definition to the output.
fn write_enum(output: &mut String, enum_: &Enum) {
    let _ = writeln!(output, "        <enum name=\"{}\">", enum_.name);

    if !enum_.description.is_empty() {
        let _ = writeln!(
            output,
            "            <description>{}</description>",
            escape_xml(enum_.description)
        );
    }

    for variant in enum_.variants {
        let _ = write!(
            output,
            "            <entry value=\"{}\" name=\"{}\"",
            variant.value, variant.name
        );

        if variant.description.is_empty() {
            output.push_str("/>\n");
        } else {
            output.push_str(">\n");
            let _ = writeln!(
                output,
                "                <description>{}</description>",
                escape_xml(variant.description)
            );
            output.push_str("            </entry>\n");
        }
    }

    output.push_str("        </enum>\n");
}

/// Writes a message definition to the output.
fn write_message(output: &mut String, message: &Message) {
    let _ = writeln!(
        output,
        "        <message id=\"{}\" name=\"{}\">",
        message.id, message.name
    );

    if !message.description.is_empty() {
        let _ = writeln!(
            output,
            "            <description>{}</description>",
            escape_xml(message.description)
        );
    }

    for field in message.fields {
        write_field(output, field);
    }

    output.push_str("        </message>\n");
}

/// Writes a field definition to the output.
fn write_field(output: &mut String, field: &MessageField) {
    let type_str = format_mav_type(field.type_);

    let _ = write!(
        output,
        "            <field type=\"{type_str}\" name=\"{}\"",
        field.name
    );

    // Add optional attributes
    if let Some(enum_) = field.enum_ {
        let _ = write!(output, " enum=\"{}\"", enum_.name);
    }

    if let Some(unit) = field.unit {
        let _ = write!(output, " units=\"{unit}\"");
    }

    // Close the field tag
    if field.description.is_empty() {
        output.push_str("/>\n");
    } else {
        output.push('>');
        output.push_str(&escape_xml(field.description));
        output.push_str("</field>\n");
    }
}

/// Formats a `MavType` as a string for XML output.
fn format_mav_type(mav_type: MavType) -> String {
    match mav_type {
        MavType::Scalar(scalar_type) => scalar_type.type_name().to_string(),
        MavType::Array(scalar_type, length) => {
            format!("{}[{}]", scalar_type.type_name(), length)
        }
    }
}

/// Escapes XML special characters in a string.
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::description::{Enum, EnumVariant, Message, MessageField};
    use crate::typing::Typeable;

    // Test helper dialects with static test data
    const TEST_MESSAGE_1: Message = Message {
        id: 1,
        name: "TEST_MESSAGE",
        description: "A test message",
        fields: &[
            MessageField {
                type_: <u8>::TYPE,
                name: "field1",
                unit: None,
                description: "First field",
                enum_: None,
            },
            MessageField {
                type_: <f32>::TYPE,
                name: "field2",
                unit: Some("m/s"),
                description: "Second field with units",
                enum_: None,
            },
        ],
    };

    const TEST_MESSAGE_2: Message = Message {
        id: 2,
        name: "ANOTHER_MESSAGE",
        description: "",
        fields: &[MessageField {
            type_: <[u8; 10]>::TYPE,
            name: "array_field",
            unit: None,
            description: "",
            enum_: None,
        }],
    };

    struct TestDialect;

    impl DescribeMessages for TestDialect {
        const MESSAGES: &[Message] = &[TEST_MESSAGE_1, TEST_MESSAGE_2];
    }

    impl mavio::protocol::MessageSpec for TestDialect {
        fn id(&self) -> u32 {
            0
        }
        fn min_supported_mavlink_version(&self) -> mavio::prelude::MavLinkVersion {
            mavio::prelude::MavLinkVersion::V2
        }
        fn crc_extra(&self) -> u8 {
            0
        }
    }

    impl mavio::protocol::IntoPayload for TestDialect {
        fn encode(
            &self,
            _version: mavio::prelude::MavLinkVersion,
        ) -> Result<mavio::protocol::Payload, mavio::error::SpecError> {
            unreachable!("not used in these tests")
        }
    }

    impl mavio::Dialect for TestDialect {
        fn name() -> &'static str {
            "test"
        }

        fn dialect() -> Option<u32> {
            Some(42)
        }

        fn version() -> Option<u8> {
            Some(1)
        }

        fn message_info(
            _id: u32,
        ) -> Result<&'static dyn mavio::protocol::MessageSpec, mavio::error::SpecError> {
            unreachable!("not used in these tests")
        }

        fn decode(_payload: &mavio::protocol::Payload) -> Result<Self, mavio::error::SpecError> {
            unreachable!("not used in these tests")
        }

        fn spec() -> &'static mavio::protocol::DialectSpec {
            unreachable!("not used in these tests")
        }
    }

    #[test]
    fn test_basic_dialect_generation() {
        let xml = make_dialect_file::<TestDialect>();

        // Check for basic structure
        assert!(xml.starts_with("<?xml version=\"1.0\"?>"));
        assert!(xml.contains("<mavlink>"));
        assert!(xml.contains("</mavlink>"));
        assert!(xml.contains("<version>1</version>"));
        assert!(xml.contains("<dialect>42</dialect>"));
        assert!(xml.contains("<messages>"));
        assert!(xml.contains("</messages>"));
    }

    #[test]
    fn test_message_generation() {
        let xml = make_dialect_file::<TestDialect>();

        // Check message structure
        assert!(xml.contains("<message id=\"1\" name=\"TEST_MESSAGE\">"));
        assert!(xml.contains("<description>A test message</description>"));
        assert!(xml.contains("</message>"));

        // Check fields
        assert!(xml.contains("<field type=\"uint8_t\" name=\"field1\">First field</field>"));
        assert!(xml.contains(
            "<field type=\"float\" name=\"field2\" units=\"m/s\">Second field with units</field>"
        ));
    }

    #[test]
    fn test_array_type_formatting() {
        let xml = make_dialect_file::<TestDialect>();

        // Check array field formatting
        assert!(xml.contains("<field type=\"uint8_t[10]\" name=\"array_field\"/>"));
    }

    #[test]
    fn test_empty_description_handling() {
        let xml = make_dialect_file::<TestDialect>();

        // Messages without description should not have empty description tags
        assert!(xml.contains("<message id=\"2\" name=\"ANOTHER_MESSAGE\">"));
        // Should not have standalone description tag after ANOTHER_MESSAGE
        if let Some(another_msg_idx) = xml.find("ANOTHER_MESSAGE")
            && let Some(next_field) = xml[another_msg_idx..].find("<field")
        {
            let msg_segment = &xml[another_msg_idx..another_msg_idx + next_field];
            assert!(!msg_segment.contains("<description></description>"));
        }
    }

    #[test]
    fn test_xml_escaping() {
        assert_eq!(escape_xml("Hello World"), "Hello World");
        assert_eq!(
            escape_xml("Test & <check> \"quotes\""),
            "Test &amp; &lt;check&gt; &quot;quotes&quot;"
        );
        assert_eq!(escape_xml("it's"), "it&apos;s");
    }

    #[test]
    fn test_enum_generation() {
        const TEST_ENUM: Enum = Enum {
            name: "TEST_ENUM",
            description: "Test enum description",
            variants: &[
                EnumVariant {
                    value: 0,
                    name: "VARIANT_A",
                    description: "First variant",
                },
                EnumVariant {
                    value: 1,
                    name: "VARIANT_B",
                    description: "",
                },
            ],
        };

        const ENUM_MESSAGE: Message = Message {
            id: 10,
            name: "ENUM_MESSAGE",
            description: "Message with enum field",
            fields: &[MessageField {
                type_: <u8>::TYPE,
                name: "enum_field",
                unit: None,
                description: "Field with enum",
                enum_: Some(&TEST_ENUM),
            }],
        };

        struct EnumTestDialect;

        impl DescribeMessages for EnumTestDialect {
            const MESSAGES: &[Message] = &[ENUM_MESSAGE];
        }

        impl mavio::protocol::MessageSpec for EnumTestDialect {
            fn id(&self) -> u32 {
                0
            }
            fn min_supported_mavlink_version(&self) -> mavio::prelude::MavLinkVersion {
                mavio::prelude::MavLinkVersion::V2
            }
            fn crc_extra(&self) -> u8 {
                0
            }
        }

        impl mavio::protocol::IntoPayload for EnumTestDialect {
            fn encode(
                &self,
                _version: mavio::prelude::MavLinkVersion,
            ) -> Result<mavio::protocol::Payload, mavio::error::SpecError> {
                unreachable!("not used in these tests")
            }
        }

        impl mavio::Dialect for EnumTestDialect {
            fn name() -> &'static str {
                "enum_test"
            }
            fn dialect() -> Option<u32> {
                None
            }
            fn version() -> Option<u8> {
                None
            }
            fn message_info(
                _id: u32,
            ) -> Result<&'static dyn mavio::protocol::MessageSpec, mavio::error::SpecError>
            {
                unreachable!("not used in these tests")
            }
            fn decode(
                _payload: &mavio::protocol::Payload,
            ) -> Result<Self, mavio::error::SpecError> {
                unreachable!("not used in these tests")
            }
            fn spec() -> &'static mavio::protocol::DialectSpec {
                unreachable!("not used in these tests")
            }
        }

        let xml = make_dialect_file::<EnumTestDialect>();

        // Check enum section exists
        assert!(xml.contains("<enums>"));
        assert!(xml.contains("</enums>"));

        // Check enum structure
        assert!(xml.contains("<enum name=\"TEST_ENUM\">"));
        assert!(xml.contains("<description>Test enum description</description>"));

        // Check variants
        assert!(xml.contains("<entry value=\"0\" name=\"VARIANT_A\">"));
        assert!(xml.contains("<description>First variant</description>"));
        assert!(xml.contains("<entry value=\"1\" name=\"VARIANT_B\"/>"));

        // Check field references enum
        assert!(xml.contains("<field type=\"uint8_t\" name=\"enum_field\" enum=\"TEST_ENUM\">"));
    }

    #[test]
    fn test_format_mav_type() {
        use crate::typing::MavScalarType;

        assert_eq!(
            format_mav_type(MavType::Scalar(MavScalarType::UInt8)),
            "uint8_t"
        );
        assert_eq!(
            format_mav_type(MavType::Scalar(MavScalarType::Float)),
            "float"
        );
        assert_eq!(
            format_mav_type(MavType::Array(MavScalarType::UInt16, 5)),
            "uint16_t[5]"
        );
        assert_eq!(
            format_mav_type(MavType::Array(MavScalarType::Char, 30)),
            "char[30]"
        );
    }
}

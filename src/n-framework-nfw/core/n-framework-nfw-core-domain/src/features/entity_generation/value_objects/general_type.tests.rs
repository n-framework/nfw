use super::GeneralType;

#[test]
fn from_cli_type_maps_string() {
    assert_eq!(
        GeneralType::from_cli_type("string"),
        Some(GeneralType::String)
    );
}

#[test]
fn from_cli_type_maps_int_and_long_to_integer() {
    assert_eq!(
        GeneralType::from_cli_type("int"),
        Some(GeneralType::Integer)
    );
    assert_eq!(
        GeneralType::from_cli_type("long"),
        Some(GeneralType::Integer)
    );
}

#[test]
fn from_cli_type_maps_decimal_variants() {
    assert_eq!(
        GeneralType::from_cli_type("decimal"),
        Some(GeneralType::Decimal)
    );
    assert_eq!(
        GeneralType::from_cli_type("double"),
        Some(GeneralType::Decimal)
    );
    assert_eq!(
        GeneralType::from_cli_type("float"),
        Some(GeneralType::Decimal)
    );
}

#[test]
fn from_cli_type_maps_bool() {
    assert_eq!(
        GeneralType::from_cli_type("bool"),
        Some(GeneralType::Boolean)
    );
}

#[test]
fn from_cli_type_maps_datetime_variants() {
    assert_eq!(
        GeneralType::from_cli_type("DateTime"),
        Some(GeneralType::DateTime)
    );
    assert_eq!(
        GeneralType::from_cli_type("DateTimeOffset"),
        Some(GeneralType::DateTime)
    );
}

#[test]
fn from_cli_type_maps_guid_to_uuid() {
    assert_eq!(GeneralType::from_cli_type("Guid"), Some(GeneralType::Uuid));
}

#[test]
fn from_cli_type_maps_byte_array() {
    assert_eq!(
        GeneralType::from_cli_type("byte[]"),
        Some(GeneralType::Bytes)
    );
}

#[test]
fn from_cli_type_rejects_unknown() {
    assert_eq!(GeneralType::from_cli_type("List<int>"), None);
    assert_eq!(GeneralType::from_cli_type("InvalidType"), None);
}

#[test]
fn display_roundtrips_through_from_str() {
    let types = vec![
        GeneralType::String,
        GeneralType::Integer,
        GeneralType::Decimal,
        GeneralType::Boolean,
        GeneralType::DateTime,
        GeneralType::Uuid,
        GeneralType::Bytes,
    ];
    for t in types {
        let s = t.to_string();
        let parsed: GeneralType = s.parse().unwrap();
        assert_eq!(parsed, t);
    }
}

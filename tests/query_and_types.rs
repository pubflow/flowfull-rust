use flowfull::{
    auth::{FlexibleBool, User},
    between, contains, gte, in_op, not_null,
    operators::format_value,
};
use pretty_assertions::assert_eq;

#[test]
fn operator_values_are_formatted_for_bracket_syntax() {
    assert_eq!(format_value(&gte(18).value), "18");
    assert_eq!(format_value(&contains("john").value), "john");
    assert_eq!(format_value(&in_op(["a", "b"]).value), "a,b");
    assert_eq!(format_value(&between(1, 10).value), "1,10");
    assert_eq!(format_value(&not_null().value), "true");
}

#[test]
fn flexible_bool_accepts_flowless_variants() {
    #[derive(serde::Deserialize)]
    struct Wrapper {
        value: FlexibleBool,
    }

    for raw in ["true", "1", "\"1\"", "\"true\""] {
        let parsed: Wrapper = serde_json::from_str(&format!("{{\"value\":{raw}}}")).unwrap();
        assert_eq!(parsed.value, FlexibleBool(true));
    }

    for raw in ["false", "0", "\"0\"", "\"false\""] {
        let parsed: Wrapper = serde_json::from_str(&format!("{{\"value\":{raw}}}")).unwrap();
        assert_eq!(parsed.value, FlexibleBool(false));
    }
}

#[test]
fn user_accepts_optional_flowless_profile_fields() {
    let user: User = serde_json::from_value(serde_json::json!({
        "id": "usr_1",
        "email": "user@example.com",
        "name": "Jane",
        "is_verified": "1",
        "two_factor": 0
    }))
    .unwrap();

    assert_eq!(user.id, "usr_1");
    assert_eq!(user.is_verified, FlexibleBool(true));
    assert_eq!(user.two_factor, FlexibleBool(false));
}
